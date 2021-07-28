use super::super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use shared::domain::jig::module::body::{ModeExt, BodyExt, StepExt};
use web_sys::AudioContext;
use crate::audio_mixer::AudioMixer;


pub fn render_loading<RawData, Mode, Step, Base> (state:Rc<GenericState<RawData, Mode, Step, Base>>) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
{
    //TODO - make this a custom element
    html!("div", {
        .style("position", "absolute")
        .style("width", "100%")
        .style("height", "100%")
        .style("background-color", "red")
        .property("slot", "main")
    })
}
