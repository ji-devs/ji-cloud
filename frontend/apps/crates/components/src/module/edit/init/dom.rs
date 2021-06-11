use super::super::{
    state::*,
    steps::state::*,
};
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use shared::domain::jig::module::body::{ModeExt, BodyExt};
use web_sys::AudioContext;
use crate::audio_mixer::AudioMixer;

pub fn render<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay> (state:Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom
where
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    RawData: BodyExt<Mode> + 'static, 
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
