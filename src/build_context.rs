use parking_lot::Mutex;
use sb_sbity::block::BlockMutationEnum;
use std::{collections::HashMap, sync::Arc};

use crate::uid::Uid;

pub struct GlobalVarListContext {
    pub vars: HashMap<String, Uid>,
    pub lists: HashMap<String, Uid>,
}

pub type CustomFuncTy = (BlockMutationEnum, HashMap<String, Uid>);

pub struct TargetContext<'a> {
    pub global_vars: &'a HashMap<String, Uid>,
    pub global_lists: &'a HashMap<String, Uid>,
    pub this_sprite_vars: &'a HashMap<String, Uid>,
    pub this_sprite_lists: &'a HashMap<String, Uid>,
    pub all_broadcasts: &'a HashMap<String, Uid>,
    pub custom_funcs: Arc<Mutex<HashMap<String, CustomFuncTy>>>,
}
