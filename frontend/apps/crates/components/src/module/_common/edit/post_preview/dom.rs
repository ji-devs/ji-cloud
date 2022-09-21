use dominator::{clone, html, Dom};

use super::state::*;
use shared::domain::module::{
    body::{BodyExt, ModeExt, StepExt},
    ModuleKind,
};
use std::rc::Rc;
use utils::{init::analytics, prelude::*};

pub fn render_post_preview<RawData, Mode, Step>(state: Rc<PostPreview>, raw_data: RawData) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    html!("post-preview-container", {
        .child(html!("post-preview", {
            .child(html!("window-loader-block", {
                .property("slot", "loader")
                .property_signal("visible", state.loader.is_loading())
            }))
            .property("module", state.module_kind.as_str())
            .property("hasConvertable", !RawData::convertable_list().is_empty())
            .children(
                RawData::convertable_list()
                    .iter()
                    .enumerate()
                    .map(|(index, kind)| {
                        html!("post-preview-action", {
                            .property("slot", format!("module-{}", index+1))
                            .property("kind", kind.as_str())
                            .event(clone!(state, kind, raw_data => move |_evt:events::Click| {
                                state.duplicate_module(kind, raw_data.clone());
                            }))
                        })
                    })
                    .collect::<Vec<Dom>>()
            )
            /* Leaving off fo now...
            .child(
                html!("post-preview-action", {
                    .property("slot", "action-print")
                    .property("kind", "print")
                    .event(clone!(state => move |evt:events::Click| {
                        log::info!("TODO - print!")
                    }))
                })
            )
            */
            .apply(|dom| {
                log::info!("{:?}", RawData::kind());
                match RawData::kind() {
                    ModuleKind::Memory | ModuleKind::Flashcards | ModuleKind::Matching | ModuleKind::CardQuiz => {
                        dom.child(html!("post-preview-action", {
                            .property("slot", "action-print")
                            .property("kind", "print-cards")
                            .event(clone!(state => move |_evt:events::Click| {
                                analytics::event("Jig Edit Print Cards", None);

                                if state.print_cards(&raw_data).is_err() {
                                    let _ = web_sys::window()
                                        .unwrap_ji()
                                        .alert_with_message("Error");
                                }
                            }))
                        }))
                    }
                    ModuleKind::Poster => {
                        dom.child(html!("post-preview-action", {
                            .property("slot", "action-print")
                            .property("kind", "print")
                            .event(clone!(state => move |_evt:events::Click| {
                                analytics::event("Jig Edit Print Design", None);
                                state.print_design();
                            }))
                        }))
                    }
                    _ => dom
                }
            })
            .child(
                html!("post-preview-action", {
                    .property("slot", "action-publish")
                    .property("kind", "publish")
                    .event(clone!(state => move |_evt:events::Click| {
                        state.publish();
                    }))
                })
            )
            .child(
                html!("post-preview-action", {
                    .property("slot", "action-continue")
                    .property("kind", "continue")
                    .event(clone!(state => move |_evt:events::Click| {
                        state.next();
                    }))
                })
            )
        }))
    })
}
