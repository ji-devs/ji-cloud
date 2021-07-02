use super::{
    super::{
        state::*,
        nav::dom::render_nav
    },
    super::super::actions::HistoryStateImpl,
    super::super::strings
};
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
use crate::module::_common::edit::header::controller::dom::ControllerDom;
use shared::domain::jig::{JigId, module::{ModuleKind, ModuleId, body::{BodyExt, ModeExt, StepExt}}};
use utils::{prelude::*, iframe::{IframeInit, EmptyMessage}}; 
use dominator_helpers::events::Message;

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
    let post_preview = state.base.get_post_preview();

        html!("module-preview-header", {
            .property("slot", "header")
            .property("moduleKind", module_kind.as_str())
            .child(render_nav(state.clone()))

            .apply_if(post_preview.is_some(), |dom| {
                let post_preview = Rc::new(post_preview.unwrap_ji());

                dom
                    .child(html!("button-rect", {
                        .property("slot", "btn")
                        .property("size", "small")
                        .property("iconAfter", "arrow")
                        .text(strings::STR_DONE)
                        .event(clone!(state, post_preview => move |evt:events::Click| {
                            state.preview_mode.set(Some(PreviewMode::PostPreview(post_preview.clone())));

                        }))
                    }))
            })
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
                    if let Ok(_) = evt.try_serde_data::<IframeInit<EmptyMessage>>() {
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
