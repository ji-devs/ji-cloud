use components::module::_common::play::prelude::*;
use shared::domain::{
    asset::{Asset, AssetId},
    jig::{
        codes::JigPlaySessionDragDrop,
        player::{ModuleConfig, Seconds},
    },
    module::{
        body::{
            ModuleAssist,
            _groups::design::Backgrounds,
            drag_drop::{
                Item, Mode, ModuleData as RawData, PlaySettings, Step, TargetArea, TargetTransform,
            },
            ModuleAssistType,
        },
        ModuleId, StableModuleId,
    },
};
use utils::prelude::*;

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use std::rc::Rc;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub stable_module_id: StableModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub instructions: ModuleAssist,
    pub feedback: ModuleAssist,
    pub feedback_signal: Mutable<Option<ModuleAssist>>,
    pub settings: PlaySettings,
    pub backgrounds: Backgrounds,
    pub items: Vec<Item>,
    pub item_targets: Vec<TargetTransform>,
    pub target_areas: Vec<TargetArea>,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub play_report: Mutable<JigPlaySessionDragDrop>,
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            stable_module_id,
            asset,
            raw,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        Rc::new(Self {
            asset_id,
            module_id,
            stable_module_id,
            asset,
            theme_id,
            instructions: content.instructions,
            feedback: content.feedback,
            feedback_signal: Mutable::new(None),
            settings: content.play_settings,
            backgrounds: content.backgrounds,
            items: content.items,
            item_targets: content.item_targets,
            target_areas: content.target_areas,
            module_phase: init_args.play_phase,
            play_report: Mutable::new(JigPlaySessionDragDrop::new(stable_module_id)),
        })
    }
}

impl BaseExt for Base {
    fn get_module_assist(&self) -> Option<ModuleAssist> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<ModuleAssist>> {
        self.feedback_signal.read_only()
    }

    fn handle_module_assist_ended(&self, module_assist_type: ModuleAssistType) {
        if let ModuleAssistType::Feedback = module_assist_type {
            self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
        }
    }

    fn get_module_config(&self) -> ModuleConfig {
        ModuleConfig {
            timer: self.settings.time_limit.map(|t| Seconds(t)),
            ..Default::default()
        }
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
