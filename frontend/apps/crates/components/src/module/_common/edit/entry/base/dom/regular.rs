use super::super::{nav::dom::render_nav, state::*};

use dominator::{clone, html, Dom};
use std::rc::Rc;

use crate::module::_common::edit::header::controller::dom::ControllerDom;

use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use utils::prelude::*;

pub fn render_main_bg<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    let dom = match Main::render_bg(state.main.clone()) {
        Some(dom) => dom,
        None => html!("empty-fragment"),
    };

    add_slot_to_dom(dom, "main-bg")
}
pub fn render_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    add_slot_to_dom(Main::render(state.main.clone()), "main")
}

pub fn render_sidebar<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    html!("module-sidebar", {
        .property("slot", "sidebar")
        .child(render_nav(state.clone()))
        .child(add_slot_to_dom(Sidebar::render(state.sidebar.clone()), "content"))
    })
}

pub fn render_header<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    html!("module-header", {
        .property("slot", "header")
        .property("moduleKind", RawData::kind().as_str())
        .child(ControllerDom::render(
            state.history.clone(),
            clone!(state => move || {
                state.try_change_step(Step::get_preview());
            })
        ))
        .child(Header::render(state.header.clone()))
    })
}

pub fn render_footer<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    html!("module-footer", {
        .property("slot", "footer")
        .child(Footer::render(state.footer.clone()))
        .child(html!("module-footer-continue-button", {
            .property("slot", "btn")
            .property_signal("enabled", state.base.next_step_allowed_signal())
            .event(clone!(state => move |_evt:events::Next| {
                state.try_next_step();
            }))
        }))
    })
}

pub fn render_overlay<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    add_slot_to_dom(Overlay::render(state.overlay.clone()), "overlay")
}

fn add_slot_to_dom(dom: Dom, slot: &str) -> Dom {
    //there might be a better way, like Dom->DomBuilder->Dom
    html!("empty-fragment", {
        .property("slot", slot)
        .child(dom)
    })
}
