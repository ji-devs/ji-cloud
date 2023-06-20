use crate::player_popup::{PlayerPopup, PreviewPopupCallbacks};

use super::super::{
    super::{super::jig::post_preview::PostPreview, strings},
    nav::dom::render_nav,
    state::*,
};
use dominator::{clone, html, with_node, Dom};
use dominator_helpers::events::Message;
use std::rc::Rc;

use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::{
    asset::{AssetId, DraftOrLive},
    module::{
        body::{BodyExt, ModeExt, StepExt},
        ModuleId, ModuleKind,
    },
};
use utils::{
    asset::{AssetPlayerOptions, JigPlayerOptions},
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
    html!("module-preview-header", {
        .prop("slot", "header")
        .prop("moduleKind", module_kind.as_str())
        .child(render_nav(state.clone()))
        .apply_if(!state.asset.is_jig(), clone!(state => move |dom| {
            dom.child(html!("button-rect", {
                .prop("slot", "btn")
                .prop("color", "red")
                .prop("kind", "filled")
                .prop("size", "regular")
                .text(strings::STR_DONE)
                .child(html!("img-ui", {
                    .prop("path", "core/buttons/rect/arrow-right-yellow.svg")
                }))
                .event(clone!(state => move |_evt:events::Click| {
                    state.navigate_to_publish();
                }))
            }))
        }))
    })
}

pub fn render_preview_overlay<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    _module_kind: ModuleKind,
    _asset_id: AssetId,
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
        .prop("slot", "overlay")
        .child_signal(state.jig_is_post_preview.signal_cloned().map(clone!(state => move |jig_is_post_preview| {
            jig_is_post_preview.then(|| {
                let data = state.history.get_current();
                PostPreview::new(
                    RawData::kind(),
                    *state.base.get_asset_id().unwrap_jig(), // only jigs should have the post preview page
                    state.base.get_module_id(),
                ).render(data)
            })
        })))
    })
}

pub fn render_preview_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    asset_id: AssetId,
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
    if state.asset.is_jig() {
        render_preview_main_jig(asset_id, module_id, state)
    } else {
        render_preview_main_non_jig(asset_id, module_id, state)
    }
}

fn render_preview_main_jig<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    asset_id: AssetId,
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
    let preview_open = Mutable::new(true);

    html!("preview-body", {
        .prop("slot", "main")
        .child(html!("button-rect", {
            .prop("slot", "actions")
            .prop("color", "red")
            .prop("kind", "text")
            .text(strings::STR_PREVIEW_AGAIN)
            .event(clone!(preview_open => move |_: events::Click| {
                preview_open.set_neq(true);
            }))
        }))
        .child(html!("button-rect", {
            .prop("slot", "actions")
            .prop("color", "red")
            .prop("kind", "filled")
            .prop("size", "regular")
            .text(strings::STR_DONE)
            .child(html!("img-ui", {
                .prop("path", "core/buttons/rect/arrow-right-yellow.svg")
            }))
            .event(clone!(state => move |_evt:events::Click| {
                state.jig_is_post_preview.set(true);
            }))
        }))
        .child_signal(preview_open.signal_cloned().map(clone!(preview_open => move |open| {
            if open {
                let close = clone!(preview_open => move || {
                    preview_open.set_neq(false);
                });
                Some(
                    PlayerPopup::new(
                        asset_id,
                        Some(module_id),
                        None,
                        AssetPlayerOptions::Jig(JigPlayerOptions {
                            draft_or_live: DraftOrLive::Draft,
                            ..Default::default()
                        }),
                        PreviewPopupCallbacks::new(close)
                    ).render(Some("popup"))
                )
            } else {
                None
            }
        })))
    })
}

fn render_preview_main_non_jig<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    asset_id: AssetId,
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
        let route: String =
            Route::Module(ModuleRoute::Play(RawData::kind(), asset_id, module_id)).into();

        let url = SETTINGS.get().unwrap_ji().remote_target.spa_iframe(&route);

        format!("{}?iframe_data=true", url)
    };

    html!("iframe" => web_sys::HtmlIFrameElement, {
        .prop("allow", "autoplay; fullscreen")
        .prop("slot", "main")
        .prop("src", url.clone())
        .style("width", "100%")
        .style("height", "100%")
        .style("border", "none")
        .with_node!(elem => {
            .global_event(clone!(state, url => move |evt:Message| {

                //Wait until the iframe sends its empty message
                //Then send back the current raw data from history
                if evt.try_serde_data::<IframeInit<EmptyMessage>>().is_ok() {
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
