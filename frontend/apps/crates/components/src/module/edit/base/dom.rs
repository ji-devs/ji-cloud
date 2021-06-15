use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom, with_node};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashSet;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use wasm_bindgen::prelude::*;
use crate::module::header::controller::dom::ControllerDom;
use super::super::actions::HistoryStateImpl;
use shared::domain::jig::{JigId, module::{ModuleKind, ModuleId, body::{BodyExt, ModeExt, StepExt}}};
use utils::{prelude::*, iframe::IframeInit}; 
use dominator_helpers::events::Message;

pub fn render<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    is_preview: bool,
    jig_id: JigId,
    module_id: ModuleId,
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>
) -> Vec<Dom>
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
    if is_preview {
        vec![
            render_preview_header(RawData::kind(), state.clone()),
            render_preview_main(RawData::kind(), jig_id, module_id, state.clone())
        ]
    } else {
        vec![
            render_main(state.clone()),
            render_sidebar(state.clone()),
            render_header(state.clone()),
            render_footer(state.clone()),
            render_overlay(state.clone()),
        ]
    }
}

pub fn render_preview_header<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    module_kind: ModuleKind,
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>
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
        html!("module-preview-header", {
            .property("slot", "header")
            .property_signal("theme", state.base.theme_id_str_signal())
            .property("moduleKind", module_kind.as_str())
            .child(super::nav::dom::render(state.clone()))
            .child(html!("button-rect", {
                .property("slot", "btn")
                .property("size", "small")
                .property("iconAfter", "arrow")
                .text(super::super::strings::STR_DONE)
                .event(clone!(state => move |evt:events::Click| {
                }))
            }))
        })
}

pub fn render_preview_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    module_kind: ModuleKind, 
    jig_id: JigId, 
    module_id: ModuleId, 
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>
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
        let url = {
            let route:String = Route::Module(ModuleRoute::Play(module_kind, jig_id, module_id)).into();

            let url = unsafe {
                SETTINGS.get_unchecked()
                    .remote_target
                    .spa_iframe(&route)
            };

            format!("{}?iframe_data=true", url)
        };


        html!("iframe" => web_sys::HtmlIFrameElement, {
            .property("slot", "main")
            .style("width", "100%")
            .style("height", "100%")
            .property("src", url.clone())
            .with_node!(elem => {
                .global_event(clone!(state, url => move |evt:Message| {

                    //Wait until the iframe sends its empty message
                    //Then send back the current raw data from history
                    if let Ok(_) = evt.try_serde_data::<IframeInit<()>>() {
                        log::info!("sending iframe message!");
                        let data = state.history.get_current();

                        let msg:IframeInit<RawData> = IframeInit::new(data); 
                        if let Some(window) = elem.content_window() {
                            window.post_message(&msg.into(), &url);
                        } else {
                            log::info!("unable to get window for sending iframe msg!");
                        }
                    } else {
                        log::info!("hmmm got other iframe message...");
                    }
                }))
            })
        })
}
pub fn render_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom 
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
    add_slot_to_dom(state.clone(), Main::render(state.main.clone()), "main")
}

pub fn render_sidebar<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom 
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
        .child(super::nav::dom::render(state.clone()))
        .child(add_slot_to_dom(state.clone(), Sidebar::render(state.sidebar.clone()), "content"))
    })
}

pub fn render_header<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom 
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
        .property_signal("theme", state.base.theme_id_str_signal())
        .child(ControllerDom::render(
            state.history.clone(),
            clone!(state => move || {
                state.try_change_step(Step::get_preview());
            })
        ))
        .child(Header::render(state.header.clone()))
    })
}

pub fn render_footer<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom 
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
        .property_signal("theme", state.base.theme_id_str_signal())
        .child(Footer::render(state.footer.clone()))
        .child(html!("module-footer-continue-button", {
            .property("slot", "btn")
            .property_signal("enabled", state.base.next_step_allowed_signal())
            .event(clone!(state => move |evt:events::Next| {
                state.try_next_step();
            }))
        }))
    })
}

pub fn render_overlay<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>) -> Dom 
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
    add_slot_to_dom(state.clone(), Overlay::render(state.overlay.clone()), "overlay")
}

fn add_slot_to_dom<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>, dom:Dom, slot:&str) -> Dom 
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
    //there might be a better way, like Dom->DomBuilder->Dom
    html!("empty-fragment", {
        .property_signal("theme", state.base.theme_id_str_signal())
        .property("slot", slot)
        .child(dom)
    })
}
