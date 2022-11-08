use dominator::{clone, html, Dom};

use super::state::*;
use shared::domain::module::{
    body::{BodyExt, ModeExt, StepExt},
    ModuleKind,
};
use std::rc::Rc;
use utils::{init::analytics, prelude::*};

impl PostPreview {
    pub fn render<RawData, Mode, Step>(self: &Rc<Self>, raw_data: RawData) -> Dom
    where
        RawData: BodyExt<Mode, Step> + 'static,
        Mode: ModeExt + 'static,
        Step: StepExt + 'static,
    {
        let state = self;
        html!("post-preview-container", {
            .child(html!("post-preview", {
                .child(html!("window-loader-block", {
                    .prop("slot", "loader")
                    .prop_signal("visible", state.loader.is_loading())
                }))
                .prop("module", state.module_kind.as_str())
                .prop("hasConvertable", !RawData::convertable_list().is_empty())
                .children(
                    RawData::convertable_list()
                        .iter()
                        .enumerate()
                        .map(|(index, kind)| {
                            html!("post-preview-action", {
                                .prop("slot", format!("module-{}", index+1))
                                .prop("kind", kind.as_str())
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
                        .prop("slot", "action-print")
                        .prop("kind", "print")
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
                                .prop("slot", "action-print")
                                .prop("kind", "print-cards")
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
                        ModuleKind::Poster | ModuleKind::Cover | ModuleKind::ResourceCover => {
                            dom.child(html!("post-preview-action", {
                                .prop("slot", "action-print")
                                .prop("kind", "print")
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
                        .prop("slot", "action-publish")
                        .prop("kind", "publish")
                        .event(clone!(state => move |_evt:events::Click| {
                            state.publish();
                        }))
                    })
                )
                .child(
                    html!("post-preview-action", {
                        .prop("slot", "action-continue")
                        .prop("kind", "continue")
                        .event(clone!(state => move |_evt:events::Click| {
                            state.next();
                        }))
                    })
                )
            }))
        })
    }
}
