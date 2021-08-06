use super::super::{base::state::*, state::*};
use dominator::{clone, html, Dom};
use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use std::rc::Rc;
use utils::prelude::*;

pub fn render<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
{
    html!("div", {
        .property("slot", "main")
        .child(html!("button", {
            .text("START")
            .event(clone!(state => move |_evt:events::Click| {
                if let Some(on_init_ready) = state.on_init_ready.borrow().as_ref() {

                    (on_init_ready) ();
                }
            }))
        }))
    })
}
