use dominator::{html, clone, Dom};
use futures_signals::signal::SignalExt;
use futures_signals::signal_vec::SignalVecExt;
use utils::prelude::*;
use super::{actions, state::*};
use std::rc::Rc;
use strum::IntoEnumIterator;


pub struct GalleryDom {
}

const STR_DELETE: &'static str = "Delete";
const STR_DUPLICATE: &'static str = "Duplicate";
const STR_SEARCH: &'static str = "Search";
const STR_SHOW_JIG_ALL: &'static str = "Show all my JIGs";
const STR_SHOW_JIG_PUBLISHED: &'static str = "Show published Jigs";
const STR_SHOW_JIG_DRAFT: &'static str = "Show drafts";


impl GalleryDom {
    fn visible_jigs_option_string(visible_jigs: &VisibleJigs) -> &'static str {
        match visible_jigs {
            VisibleJigs::All => STR_SHOW_JIG_ALL,
            VisibleJigs::Published => STR_SHOW_JIG_PUBLISHED,
            VisibleJigs::Draft => STR_SHOW_JIG_DRAFT,
        }
    }

    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_jigs(state.clone());

        html!("jig-gallery", {
            .child(html!("jig-gallery-create", {
                .property("slot", "create-jig")
                .event(clone!(state => move |_: events::Click| {
                    actions::create_jig(state.clone());
                }))
            }))
            .children(TEMPLATE_KINDS.iter().map(|kind| {
                html!("jig-gallery-template", {
                    .property("slot", "jig-templates")
                    .property("kind", *kind)
                })
            }))
            .child(html!("input-search", {
                .property("slot", "search-input")
                .property("placeholder", STR_SEARCH)
                .event(clone!(state => move |evt: events::CustomSearch| {
                    let value = evt.query();
                    if !value.is_empty() {
                        actions::search_jigs(state.clone(), value);
                    } else {
                        actions::load_jigs(state.clone());
                    }
                }))
            }))
            .child(html!("dropdown-select", {
                .property("slot", "filters")
                .property_signal("value", state.visible_jigs.signal_cloned().map(|visible_jigs| Self::visible_jigs_option_string(&visible_jigs)))
                .children(VisibleJigs::iter().map(|option| {
                    html!("li-check", {
                        .property("value", &option.to_string())
                        .text(Self::visible_jigs_option_string(&option))
                        .property_signal("selected", state.visible_jigs.signal_cloned().map(clone!(state, option => move |visible_jigs| {
                            if visible_jigs == option {
                                true
                            } else {
                                false
                            }
                        })))
                        .event(clone!(state, option => move |_: events::Click| {
                            state.visible_jigs.set(option.clone());
                            actions::load_jigs(state.clone());
                        }))
                    })
                }))
            }))
            // todo: deal with loading
            // .child(html!("window-loader-block", {
            //     .property("slot", "recent-items")
            //     .property_signal("visible", state.loader.is_loading())
            // }))
            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |jig| {
                html!("jig-gallery-recent", {
                    .property("slot", "recent-items")
                    .property("href", jig.id.0.to_string())
                    .property("draft", "")
                    .property("label", jig.display_name.clone().unwrap_or_default())
                    .property("img", "mock/resized/jig-gallery.jpg")
                    .property("ages", "5-8")
                    .property("lastedited", "3 W ago")
                    .children(&mut [
                        html!("menu-line", {
                            .property("slot", "menu-content")
                            .property("icon", "duplicate")
                            .text(STR_DUPLICATE)
                            .event(clone!(state, jig => move |_: events::Click| {
                                actions::copy_jig(state.clone(), &jig);
                            }))
                        }),
                        html!("menu-line", {
                            .property("slot", "menu-content")
                            .property("icon", "delete")
                            .text(STR_DELETE)
                            .event(clone!(state, jig => move |_: events::Click| {
                                actions::delete_jig(state.clone(), jig.id);
                            }))
                        }),
                    ])
                })
            })))
        })
    }
}
