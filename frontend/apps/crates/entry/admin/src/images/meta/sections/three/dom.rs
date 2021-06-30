use dominator::{html, clone, Dom};
use std::rc::Rc;
use crate::images::meta::{
    state::{State as MetaState, MutableImage},
    sections::common::categories::MutableCategory
};
use super::{state::*, actions};
use crate::images::meta::{state::Section, sections::common::categories::*};
use utils::events;
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::meta::MetadataResponse;

pub struct Section3Dom {
}

impl Section3Dom {
    pub fn render(meta_state: Rc<MetaState>, image: Rc<MutableImage>, metadata: Rc<MetadataResponse>, categories: Rc<Vec<Rc<MutableCategory>>>) -> Dom {
        let state = Rc::new(State::new(meta_state, image, metadata, categories));

        html!("image-meta-section-3", {
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
                })
            ])
        })
    }
}
