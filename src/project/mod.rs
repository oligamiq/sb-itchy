use std::collections::HashMap;

use crate::{
    resource::Resource,
    target::{SpriteBuilder, StageBuilder},
    uid::Uid,
};
use sb_sbity::{
    monitor::Monitor,
    project::{Meta, Project},
    target::SpriteOrStage,
};

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectBuilder {
    pub stage_builder:   StageBuilder,
    pub sprite_builders: Vec<SpriteBuilder>,
    pub monitors:        Vec<Monitor>,
    pub meta:            Meta,
}

impl ProjectBuilder {
    pub fn set_stage(&mut self, stage_builder: StageBuilder) -> &mut Self {
        self.stage_builder = stage_builder;
        self
    }

    pub fn add_sprite(&mut self, sprite_builder: SpriteBuilder) -> &mut Self {
        self.sprite_builders.push(sprite_builder);
        self
    }
}

impl ProjectBuilder {
    pub fn build(self, res_buf: &mut Vec<Resource>) -> Project {
        let ProjectBuilder {
            stage_builder,
            sprite_builders,
            monitors,
            meta,
        } = self;

        let all_broadcasts: HashMap<String, Uid> = stage_builder
            .target
            .broadcasts
            .iter()
            .chain(sprite_builders.iter().flat_map(|sb| &sb.target.broadcasts))
            .map(|(name, uid)| (name.clone(), uid.clone()))
            .collect::<HashMap<_, _>>();

        let mut targets = Vec::with_capacity(1 + sprite_builders.len());
        let (stage, global_varlist_buf) = stage_builder.build(res_buf, &all_broadcasts);
        targets.push(SpriteOrStage::Stage(stage));
        targets.extend(sprite_builders.into_iter().map(|sprite_builder| {
            SpriteOrStage::Sprite(sprite_builder.build(
                res_buf,
                &global_varlist_buf,
                &all_broadcasts,
            ))
        }));
        Project {
            meta,
            extensions: serde_json::value::Value::Array(vec![]),
            monitors,
            targets,
        }
    }
}

impl Default for ProjectBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        ProjectBuilder {
            stage_builder:   StageBuilder::default(),
            sprite_builders: Vec::default(),
            monitors:        Vec::default(),
            meta: Meta {
                semver: "3.0.0".to_owned(),
                vm:     "0.2.0-prerelease.20220222132735".to_owned(),
                agent:  "sb-itchy/0.1.0".to_owned(),
            },
        }
    }
}
