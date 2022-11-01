use crate::player_popup::{PlayerPopup, PreviewPopupCallbacks};

use super::super::{
    super::{super::jig::post_preview::PostPreview, strings},
    nav::dom::render_nav,
    state::*,
};
use dominator::{clone, html, Dom};
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
        .property("slot", "header")
        .property("moduleKind", module_kind.as_str())
        .child(render_nav(state.clone()))
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
        .property("slot", "overlay")
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
    let preview_open = Mutable::new(true);

    html!("preview-body", {
        .property("slot", "main")
        .child(html!("button-rect", {
            .property("slot", "actions")
            .property("color", "red")
            .property("kind", "text")
            .text(strings::STR_PREVIEW_AGAIN)
            .event(clone!(preview_open => move |_: events::Click| {
                preview_open.set_neq(true);
            }))
        }))
        .child(html!("button-rect", {
            .property("slot", "actions")
            .property("color", "red")
            .property("kind", "filled")
            .property("size", "small")
            .text(strings::STR_DONE)
            .child(html!("img-ui", {
                .property("path", "core/buttons/rect/arrow-right-yellow.svg")
            }))
            .event(clone!(state => move |_evt:events::Click| {
                if state.asset.is_jig() {
                    state.jig_is_post_preview.set(true);
                } else {
                    state.navigate_to_publish();
                }
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
