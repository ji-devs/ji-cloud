use dominator::{html, clone, Dom};
use std::rc::Rc;
use crate::images::meta::{
    state::{State as MetaState, MutableImage},
    sections::common::categories::MutableCategory
};
use super::{state::*, actions::{self, DateTimeStrings}};
use crate::images::meta::{state::Section, sections::common::categories::*};
use utils::events;
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt
};
use shared::domain::meta::MetadataResponse;
use components::image::tag::ImageTag;

pub struct Section3Dom {
}

//Latest updated doesn't reflect changes made while viewing at the same time, e.g. changing name while viewing summary page

impl Section3Dom {
    pub fn render(meta_state: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>, categories: Rc<Vec<Rc<MutableCategory>>>) -> Dom {
        let state = Rc::new(State::new(meta_state, image, metadata, categories));
        
        let id = state.image.orig.id.clone();

        let date_time_strings:Mutable<Option<DateTimeStrings>> = Mutable::new(None);

        html!("image-meta-section-3", {
            .future(clone!(date_time_strings => async move {
                date_time_strings.set(Some(actions::load_date_time_strings(id).await));
            }))
            .children(&mut [
                html!("div", {
                    .property("slot", "category-report")
                    .children(state.categories.iter().map(|cat| {
                        render_report(state.image.categories.clone(), None, cat.clone())
                    }))
                }),
                html!("button-rect", {
                      .property("kind", "text")
                      .property("color", "blue")
                      .property("weight", "medium")
                      .property("slot", "edit")
                      .text(crate::strings::STR_EDIT)
                      .event(clone!(state => move |evt:events::Click| {
                          state.meta.section.set(Section::One);
                      }))
                }),
                html!("title-ji", {
                    .property("color", "black")
                    .property("slot", "imagename")
                    .text_signal(state.image.name.signal_cloned())
                }),
                html!("title-ji", {
                    .property("color", "black")
                    .property("slot", "description")
                    .text_signal(state.image.description.signal_cloned())
                }),
                html!("div", {
                    .property("slot", "style")
                    .children_signal_vec(
                        state.styles()
                            .map(|text| {
                                html!("title-ji", {
                                    .property("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .property("slot", "age")
                    .children_signal_vec(
                        state.age_ranges()
                            .map(|text| {
                                html!("title-ji", {
                                    .property("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .property("slot", "stream")
                    .children_signal_vec(
                        state.affiliations()
                            .map(|text| {
                                html!("title-ji", {
                                    .property("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),
                html!("div", {
                    .property("slot", "tags")
                    .children_signal_vec(
                        state.tags()
                            .map(|text| {
                                html!("title-ji", {
                                    .property("color", "black")
                                    .text(&text)
                                })
                            })
                    )
                }),

                html!("div", {
                    .property("slot", "date-time")
                    .children_signal_vec(
                        date_time_strings.signal_cloned()
                            .map(|x| {
                                match x {
                                    Some(x) => {
                                        vec![
                                            html!("title-ji", {
                                                .property("color", "black")
                                                .text(&format!("Created: {}", x.created))
                                            }),
                                            html!("title-ji", {
                                                .property("color", "black")
                                                .text(&format!("Publish: {}", x.publish))
                                            }),
                                            html!("title-ji", {
                                                .property("color", "black")
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
