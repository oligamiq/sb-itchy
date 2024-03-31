use std::collections::HashMap;

use sb_sbity::{
    block::{
        Block,
        BlockField::WithId,
        BlockInput, BlockMutation,
        BlockMutationEnum::{self, ProceduresPrototype},
        BlockNormal, ShadowInputType, UidOrValue,
    },
    comment::Comment,
    string_hashmap::StringHashMap,
    value::{Number, Value, ValueWithBool},
};

use crate::{
    build_context::{CustomFuncTy, TargetContext},
    prelude::{BlockInputBuilder, CommentBuilder, Uid},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomFuncBuilder {
    comment: Option<CommentBuilder>,
    args: Vec<CustomFuncInputType>,
    warp: bool,
    x: Option<f64>,
    y: Option<f64>,
}

impl CustomFuncBuilder {
    pub fn new() -> CustomFuncBuilder {
        CustomFuncBuilder {
            ..Default::default()
        }
    }

    pub fn add_arg(&mut self, arg: CustomFuncInputType) -> &mut Self {
        self.args.push(arg);
        self
    }

    pub fn set_comment(&mut self, comment: CommentBuilder) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_args(&mut self, args: Vec<CustomFuncInputType>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn set_warp(&mut self, warp: bool) -> &mut Self {
        self.warp = warp;
        self
    }

    pub fn set_x(&mut self, x: f64) -> &mut Self {
        self.x = Some(x);
        self
    }

    pub fn set_y(&mut self, y: f64) -> &mut Self {
        self.y = Some(y);
        self
    }

    pub fn set_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
        let CustomFuncBuilder {
            comment,
            args,
            warp,
            x,
            y,
        } = self;

        let (params_id, custom_func_ty) =
            generate_func_input_block(my_uid.clone(), &args, final_stack, warp);

        let inputs = {
            let mut inputs = StringHashMap::default();
            inputs.0.insert(
                "custom_block".into(),
                BlockInput {
                    shadow: ShadowInputType::Shadow,
                    inputs: vec![Some(UidOrValue::Uid(params_id.into_inner()))],
                },
            );
            inputs
        };

        let comment = match comment {
            Some(comment) => {
                let comment_uid = Uid::generate();
                let mut comment = comment.build();
                comment.block_id = Some(my_uid.clone().into_inner());
                comment_buff.insert(comment_uid.clone(), comment);
                Some(comment_uid.into_inner())
            }
            None => None,
        };

        let block = BlockNormal {
            opcode: "procedures_definition".into(),
            next: None,
            parent: None,
            shadow: false,
            top_level: true,
            x: x.map(|x| Number::Float(x)),
            y: y.map(|y| Number::Float(y)),
            inputs,
            fields: StringHashMap::default(),
            mutation: None,
            comment,
        };

        let func_name = args
            .first()
            .map(|arg| match arg {
                CustomFuncInputType::Text(name) => name.clone(),
                _ => "".into(),
            })
            .unwrap_or_default();

        target_context
            .custom_funcs
            .lock()
            .insert(func_name, custom_func_ty);

        block
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CustomFuncInputType {
    Text(String),
    StringOrNumber(String),
    Boolean(String),
}

pub fn generate_func_input_block(
    parent: Uid,
    params: &Vec<CustomFuncInputType>,
    stack: &mut HashMap<Uid, Block>,
    warp: bool,
) -> (Uid, CustomFuncTy) {
    let this_block_id = Uid::generate();

    let mut inputs = StringHashMap::default();

    let mut argumentids = Vec::new();
    let mut argumentnames = Vec::new();
    let mut argumentdefaults = Vec::new();

    // let mut proccode: Option<String> = None;
    let mut proccode: Option<String> = Some("".into());

    let mut custom_func_ty = HashMap::new();

    for ty in params {
        let wrapper_id = Uid::generate();
        match ty {
            CustomFuncInputType::Text(name) => {
                custom_func_ty.insert(name.clone(), wrapper_id.clone());
                match &mut proccode {
                    Some(proccode) => {
                        proccode.push_str(&format!(" {name}"));
                    }
                    None => {
                        proccode = Some(name.clone());
                    }
                }
            }
            CustomFuncInputType::StringOrNumber(name) => {
                custom_func_ty.insert(name.clone(), wrapper_id.clone());
                let (id, block, ty, default) = generate_func_input_block_var_string_number(
                    this_block_id.clone(),
                    name.clone(),
                );
                inputs.0.insert(
                    wrapper_id.clone().into_inner(),
                    BlockInput {
                        shadow: ShadowInputType::Shadow,
                        inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
                    },
                );
                argumentids.push(wrapper_id.into_inner());
                argumentnames.push(name.clone());
                argumentdefaults.push(default);
                stack.insert(id, block);
                match &mut proccode {
                    Some(proccode) => {
                        proccode.push_str(&format!(" {ty}"));
                    }
                    None => {
                        proccode = Some(ty);
                    }
                }
            }
            CustomFuncInputType::Boolean(name) => {
                custom_func_ty.insert(name.clone(), wrapper_id.clone());
                let (id, block, ty, default) = generate_func_input_block_var_boolean(
                    this_block_id.clone().into_inner(),
                    name.clone(),
                );
                inputs.0.insert(
                    wrapper_id.clone().into_inner(),
                    BlockInput {
                        shadow: ShadowInputType::Shadow,
                        inputs: vec![Some(UidOrValue::Uid(id.clone().into_inner()))],
                    },
                );
                argumentids.push(wrapper_id.into_inner());
                argumentnames.push(name.clone());
                argumentdefaults.push(default);
                stack.insert(id, block);
                match &mut proccode {
                    Some(proccode) => {
                        proccode.push_str(&format!(" {ty}"));
                    }
                    None => {
                        proccode = Some(ty);
                    }
                }
            }
        }
    }

    let block = BlockNormal {
        opcode: "procedures_prototype".into(),
        next: None,
        parent: Some(parent.into_inner()),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: inputs,
        fields: StringHashMap::default(),
        mutation: Some(BlockMutation {
            tag_name: "mutation".into(),
            children: vec![],
            mutation_enum: ProceduresPrototype {
                proccode: proccode.clone().unwrap(),
                argumentids: argumentids.clone(),
                argumentnames,
                argumentdefaults,
                warp: Some(warp),
            },
        }),
        comment: None,
    };
    stack.insert(this_block_id.clone(), Block::Normal(block));
    (
        this_block_id,
        (
            BlockMutationEnum::ProceduresCall {
                proccode: proccode.unwrap(),
                argumentids,
                warp: Some(warp),
            },
            custom_func_ty,
        ),
    )
}

pub fn generate_func_input_block_var_boolean(
    parent: String,
    name: String,
) -> (Uid, Block, String, ValueWithBool) {
    let this_block_id = Uid::generate();

    let mut fields = StringHashMap::default();

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Value::Text(name.clone()),
        },
    );

    let block = BlockNormal {
        opcode: "argument_reporter_boolean".into(),
        next: None,
        parent: Some(parent),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: StringHashMap::default(),
        fields: fields,
        mutation: None,
        comment: None,
    };
    (
        this_block_id,
        Block::Normal(block),
        "%b".into(),
        ValueWithBool::Bool(false),
    )
}

pub fn generate_func_input_block_var_string_number(
    parent: Uid,
    name: String,
) -> (Uid, Block, String, ValueWithBool) {
    let this_block_id = Uid::generate();

    let mut fields = StringHashMap::default();

    fields.0.insert(
        "VALUE".into(),
        WithId {
            id: None,
            value: Value::Text(name.clone()),
        },
    );

    let block = BlockNormal {
        opcode: "argument_reporter_string_number".into(),
        next: None,
        parent: Some(parent.into_inner()),
        shadow: true,
        top_level: false,
        x: None,
        y: None,
        inputs: StringHashMap::default(),
        fields: fields,
        mutation: None,
        comment: None,
    };
    (
        this_block_id,
        Block::Normal(block),
        "%s".into(),
        ValueWithBool::Text("".into()),
    )
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CustomFuncCallBuilder {
    name: String,
    comment: Option<CommentBuilder>,
    args: HashMap<String, BlockInputBuilder>,
    shadow: bool,
    x: Option<f64>,
    y: Option<f64>,
}

impl CustomFuncCallBuilder {
    pub fn new() -> CustomFuncCallBuilder {
        CustomFuncCallBuilder {
            ..Default::default()
        }
    }

    pub fn add_input<K: Into<String>>(&mut self, key: K, input: BlockInputBuilder) -> &mut Self {
        self.args.insert(key.into(), input);
        self
    }

    pub fn set_comment(&mut self, comment: CommentBuilder) -> &mut Self {
        self.comment = Some(comment);
        self
    }

    pub fn set_args(&mut self, args: HashMap<String, BlockInputBuilder>) -> &mut Self {
        self.args = args;
        self
    }

    pub fn set_shadow(&mut self, is_shadow: bool) -> &mut Self {
        self.shadow = is_shadow;
        self
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn set_x(&mut self, x: Option<f64>) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: Option<f64>) -> &mut Self {
        self.y = y;
        self
    }

    pub fn set_pos(&mut self, x: Option<f64>, y: Option<f64>) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
        let CustomFuncCallBuilder {
            name,
            comment,
            args,
            shadow,
            x,
            y,
        } = self;

        let (func_ty, ty) = target_context
            .custom_funcs
            .lock()
            .get(&name)
            .expect("Custom function not found")
            .clone();

        let inputs = args
            .into_iter()
            .map(|(key, input)| {
                let arg = ty.get(&key).expect("Argument not found");

                let input = input.build(my_uid, comment_buff, final_stack, target_context);
                (arg.clone().into_inner(), input)
            })
            .collect::<HashMap<_, _>>();

        let comment = match comment {
            Some(comment) => {
                let comment_uid = Uid::generate();
                let mut comment = comment.build();
                comment.block_id = Some(my_uid.clone().into_inner());
                comment_buff.insert(comment_uid.clone(), comment);
                Some(comment_uid.into_inner())
            }
            None => None,
        };

        let block = BlockNormal {
            opcode: "procedures_call".into(),
            comment,
            next: None,
            parent: None,
            inputs: StringHashMap(inputs),
            fields: StringHashMap::default(),
            shadow,
            top_level: false,
            mutation: Some(BlockMutation {
                tag_name: "mutation".into(),
                children: vec![],
                mutation_enum: func_ty,
            }),
            x: x.map(|x| Number::Float(x)),
            y: y.map(|y| Number::Float(y)),
        };

        block
    }
}
