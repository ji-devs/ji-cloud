use super::super::{
    super::{super::post_preview::{state::PostPreview, dom::render_post_preview}, strings},
    nav::dom::render_nav,
    state::*,
};
use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;

use futures_signals::signal::SignalExt;
use dominator_helpers::events::Message;
use shared::domain::jig::{
    module::{
        body::{BodyExt, ModeExt, StepExt},
        ModuleId, ModuleKind,
    },
    JigId,
};
use utils::{
    iframe::{EmptyMessage, IframeInit},
    prelude::*,
};

pub fn render_preview_header<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    module_kind: ModuleKind,
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

    let post_preview = Rc::new(PostPreview::new(
        RawData::kind(),
        state.base.get_jig_id(),
        state.base.get_module_id(),
    ));

    html!("module-preview-header", {
        .property("slot", "header")
        .property("moduleKind", module_kind.as_str())
        .child(render_nav(state.clone()))
        .child(html!("button-rect", {
            .property("slot", "btn")
            .property("size", "small")
            .property("iconAfter", "arrow")
            .text(strings::STR_DONE)
            .event(clone!(state, post_preview => move |_evt:events::Click| {
                state.preview_mode.set(Some(PreviewMode::PostPreview(post_preview.clone())));

            }))
        }))
    })
}

pub fn render_preview_overlay<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    _module_kind: ModuleKind,
    _jig_id: JigId,
    _module_id: ModuleId,
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
    html!("empty-fragment", {
        .property("slot", "overlay")
        .child_signal(state.preview_mode.signal_cloned().map(clone!(state => move |preview_mode| {
            preview_mode.and_then(|preview_mode| {
                match preview_mode {
                    PreviewMode::PostPreview(post_preview_state) => {
                        let data = state.history.get_current();
                        Some(render_post_preview(post_preview_state.clone(), data))
                    }
                    _ => None
                }
            })
        })))
    })
}

pub fn render_preview_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    module_kind: ModuleKind,
    jig_id: JigId,
    module_id: ModuleId,
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
    let url = {
        let route: String = Route::Module(ModuleRoute::Play(module_kind, jig_id, module_id)).into();

        let url = unsafe { SETTINGS.get_unchecked().remote_target.spa_iframe(&route) };

        format!("{}?iframe_data=true", url)
    };

    html!("iframe" => web_sys::HtmlIFrameElement, {
        .property("allow", "autoplay; fullscreen")
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
                        let _ = window.post_message(&msg.into(), &url);
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
