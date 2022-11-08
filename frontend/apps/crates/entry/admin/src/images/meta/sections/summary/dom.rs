use super::{
    actions::{self, DateTimeStrings},
    state::*,
};
use crate::images::meta::{
    sections::common::categories::MutableCategory,
    state::{MutableImage, State as MetaState},
};
use crate::images::meta::{sections::common::categories::*, state::Section};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::meta::MetadataResponse;
use std::rc::Rc;
use utils::events;

pub struct SummaryDom {}

//Latest updated doesn't reflect changes made while viewing at the same time, e.g. changing name while viewing summary page

impl SummaryDom {
    pub fn render(
        meta_state: Rc<MetaState>,
        image: Rc<MutableImage>,
        metadata: Rc<MetadataResponse>,
        categories: Rc<Vec<Rc<MutableCategory>>>,
    ) -> Dom {
        let state = Rc::new(State::new(meta_state, image, metadata, categories));

        let id = state.image.orig.id;

        let date_time_strings: Mutable<Option<DateTimeStrings>> = Mutable::new(None);

        html!("image-meta-section-summary", {
            .future(clone!(date_time_strings => async move {
                date_time_strings.set(Some(actions::load_date_time_strings(id).await));
            }))
            .children(&mut [
                html!("div", {
                    .prop("slot", "category-report")
                    .children(state.categories.iter().map(|cat| {
                        render_report(state.image.categories.clone(), None, cat.clone())
                    }))
                }),
                html!("button-rect", {
                      .prop("kind", "text")
                      .prop("color", "blue")
                      .prop("weight", "medium")
                      .prop("slot", "edit-general")
                      .text(crate::strings::STR_EDIT)
                      .event(clone!(state => move |_evt:events::Click| {
                          state.meta.section.set(Section::General);
                      }))
                }),
                html!("button-rect", {
                      .prop("kind", "text")
                      .prop("color", "blue")
                      .prop("weight", "medium")
                      .prop("slot", "edit-categories")
                      .text(crate::strings::STR_EDIT)
                      .event(clone!(state => move |_evt:events::Click| {
                          state.meta.section.set(Section::Categories);
                      }))
                }),
                html!("title-ji", {
                    .prop("color", "black")
                    .prop("slot", "imagename")
                    .text_signal(state.image.name.signal_cloned())
                }),
                html!("title-ji", {
                    .prop("color", "black")
                    .prop("slot", "description")
                    .text_signal(state.image.description.signal_cloned())
                }),
                html!("div", {
                    .prop("slot", "style")
                    .children_signal_vec(
                        state.styles()
                            .map(|text| {
                                html!("title-ji", {
                                    .prop("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .prop("slot", "age")
                    .children_signal_vec(
                        state.age_ranges()
                            .map(|text| {
                                html!("title-ji", {
                                    .prop("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .prop("slot", "stream")
                    .children_signal_vec(
                        state.affiliations()
                            .map(|text| {
                                html!("title-ji", {
                                    .prop("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .prop("slot", "tags")
                    .children_signal_vec(
                        state.tags()
                            .map(|text| {
                                html!("title-ji", {
                                    .prop("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),

                html!("div", {
                    .prop("slot", "date-time")
                    .children_signal_vec(
                        date_time_strings.signal_cloned()
                            .map(|x| {
                                match x {
                                    Some(x) => {
                                        vec![
                                            html!("title-ji", {
                                                .prop("color", "black")
                                                .text(&format!("Created: {}", x.created))
                                            }),
                                            html!("title-ji", {
                                                .prop("color", "black")
                                                .text(&format!("Publish: {}", x.publish))
                                            }),
                                            html!("title-ji", {
                                                .prop("color", "black")
                                                .text(&format!("Updated: {}", x.updated))
                                            }),
                                        ]
                                    },
                                    None => Vec::new()
                                }
                            })
                            .to_signal_vec()
                    )
                }),
            ])
        })
    }
}
