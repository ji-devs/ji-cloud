use super::super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use shared::domain::jig::module::body::BodyExt;
use web_sys::AudioContext;
use crate::audio_mixer::AudioMixer;


pub fn render<RawData, RawMode, Base> (state:Rc<GenericState<RawData, RawMode, Base>>) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<RawMode> + 'static, 
{
    html!("div", {
        .property("slot", "main")
        .child(html!("button", {
            .text("START")
            .event(clone!(state => move |evt:events::Click| {
                if let Some(on_init_ready) = state.on_init_ready.borrow().as_ref() {
                    (on_init_ready) ();
                }
            }))
        }))
    })
}
