use super::super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use shared::domain::jig::module::body::{ModeExt, BodyExt, StepExt};
use web_sys::AudioContext;
use crate::audio_mixer::AudioMixer;


pub fn render<RawData, Mode, Step, Base> (state:Rc<GenericState<RawData, Mode, Step, Base>>) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static, 
    Mode: ModeExt + 'static,
    Step: StepExt + 'static
{
    //TODO - make this a custom element
    html!("div", {
        .style("position", "absolute")
        .style("top", "calc((100vh - 200rem)/2)")
        .style("left", "calc((100vw - 300rem)/2)")
        .property("slot", "main")
        .child(html!("button", {
            .style("width", "300rem")
            .style("height", "200rem")
            .text("START")
            .event(clone!(state => move |evt:events::Click| {
                if let Some(on_init_ready) = state.on_init_ready.borrow().as_ref() {
                    (on_init_ready) ();
                }
            }))
        }))
    })
}
