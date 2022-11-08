use std::rc::Rc;

use discard::Discard;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};

use dominator::{clone, html};

use utils::{events::ModuleResizeEvent, resize::*};

use super::base::state::*;
use super::state::*;
use crate::{module::_common::edit::prelude::*, overlay::container::OverlayContainer};
use shared::domain::module::body::{BodyExt, ModeExt, StepExt};

pub fn render_page_body<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>>,
) where
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
    let sig = map_ref! {
        let phase = state.phase.signal_cloned(),
        let step = state.step_signal(),
        let is_post_preview = state.is_post_preview_signal()
            => {
                (phase.clone(), *step, *is_post_preview)
            }
    };

    let sig = sig.map(clone!(state => move |(phase, step, is_post_preview)| {
        let page_kind = {
            match phase.as_ref() {
                Phase::Init | Phase::Choose(_) => ModulePageKind::GridPlain,
                Phase::Base(_) => {
                    match step {
                        Some(step) if step.is_preview() => {
                            match is_post_preview {
                                false => {
                                    ModulePageKind::GridResizePreview
                                },
                                true => {
                                    ModulePageKind::GridPlain
                                },
                            }
                        }
                        _ => {
                            if state.opts.is_main_scrollable {
                                ModulePageKind::GridResizeScrollable
                            } else {
                                ModulePageKind::GridResize
                            }
                        }
                    }
                }
            }
        };

        let has_resized_once = Mutable::new(!page_kind.is_resize());

        html!(page_kind.element_name(), {
                .apply_if(page_kind.add_scrollable_attribute(), |dom| {
                    dom.prop("scrollable", true)
                })
                .event(clone!(has_resized_once => move |event:ModuleResizeEvent| {
                    //in utils / global static
                    set_resize_info(event.data());
                    has_resized_once.set_neq(true);
                }))
                .children_signal_vec({
                    has_resized_once.signal()
                        .map(clone!(state, phase => move |has_resized_once| {
                            if !has_resized_once {
                                vec![]
                            } else {
                                match phase.as_ref() {
                                    Phase::Choose(choose) => {
                                        super::choose::dom::render(choose.clone())
                                    },
                                    Phase::Base(app_base) => {
                                        super::base::dom::render(
                                            state.opts.asset_id,
                                            state.opts.module_id,
                                            app_base.clone()
                                        )
                                    },
                                    Phase::Init => {
                                        vec![super::init::dom::render(state.clone())]
                                    }
                                }
                            }
                        }))
                        .to_signal_vec()
                })
                .child(OverlayContainer::new().render(Some("overlay")))
        })
    }));

    state
        .page_body_switcher
        .load(sig.for_each(clone!(state => move |dom| {
            {
                // Discard the previous body and set the current handle to None.
                // This forces dominator to release all references held by this handle.
                let current_handle = state.dom_body_handle.replace(None);
                if let Some(current_handle) = current_handle {
                    current_handle.discard();
                }
            }

            // Append the new body and set the handle.
            let handle = dominator::append_dom(&dominator::get_id("root"), dom);
            state.dom_body_handle.set(Some(handle));
            async move {}
        })));
}
