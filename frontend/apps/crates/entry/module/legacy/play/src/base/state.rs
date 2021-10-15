use shared::domain::jig::{JigData, JigId, module::{ModuleId, body::{_groups::design::{Backgrounds, Sticker}, ThemeChoice, Instructions, legacy::{slide::Slide, ModuleData as RawData}}}};
use components::{audio::mixer::AudioMixer, module::_common::play::prelude::*};
use utils::prelude::*;
use web_sys::AudioContext;
use std::rc::Rc;
use futures_signals::signal::Mutable;
use awsm_web::loaders::fetch::fetch_url;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub theme_id: ThemeId,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub game_id: String,
    pub slide_id: String,
    pub slide: Slide,
}

impl Base {

    pub async fn new(init_args: InitFromRawArgs<RawData, (), ()>) -> Rc<Self> {

        let InitFromRawArgs {
            jig_id,
            module_id,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let url = utils::path::legacy_cdn_url(format!("{}/jigzi/slides/{}.json", raw.game_id, raw.slide_id));

        let slide:Slide = fetch_url(&url)
            .await
            .unwrap_ji()
            .json_from_str()
            .await
            .unwrap_ji();

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            module_phase: init_args.play_phase,
            game_id: raw.game_id,
            slide_id: raw.slide_id,
            slide,
        })
    }

    pub fn layers_url<T: AsRef<str>>(&self, path:T) -> String {
        self.slide_url(format!("layers/{}", path.as_ref()))
    }

    pub fn slide_url<T: AsRef<str>>(&self, path:T) -> String {
        utils::path::legacy_cdn_url(&format!("{}/unzipped/{}/{}", self.game_id, self.slide_id, path.as_ref()))
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        None
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
