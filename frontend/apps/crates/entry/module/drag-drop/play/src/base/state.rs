use components::module::_common::play::prelude::*;
use shared::domain::jig::{
    module::{
        body::{
            Instructions,
            _groups::design::Backgrounds,
            drag_drop::{
                Item, Mode, ModuleData as RawData, PlaySettings, Step, TargetArea, TargetTransform,
            },
        },
        ModuleId,
    },
    JigData, JigId,
};
use utils::prelude::*;

use futures_signals::signal::Mutable;
use std::rc::Rc;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub theme_id: ThemeId,
    pub instructions: Instructions,
    pub feedback: Instructions,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub items: Vec<Item>,
    pub item_targets: Vec<TargetTransform>,
    pub target_areas: Vec<TargetArea>,
    pub module_phase: Mutable<ModulePlayPhase>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            jig_id,
            module_id,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            instructions: content.instructions,
            feedback: content.feedback,
            settings: content.play_settings,
            backgrounds: content.backgrounds,
            items: content.items,
            item_targets: content.item_targets,
            target_areas: content.target_areas,
            module_phase: init_args.play_phase,
        })
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        Some(self.instructions.clone())
    }

    fn get_timer_minutes(&self) -> Option<u32> {
        self.settings.time_limit
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
