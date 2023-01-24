use crate::image_table::mass_editing::MassEditing;

use super::{editable_image::EditableImage, state::*};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{
    map_ref,
    signal::{from_future, Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::{
    domain::{
        category::CategoryId,
        meta::{AffiliationId, AgeRangeId, ImageStyleId, ImageTagIndex},
    },
    media::MediaLibrary,
};
use std::rc::Rc;
use utils::{
    component::Component,
    events,
    metadata::{
        get_affiliations, get_age_ranges, get_category_label_lookup, get_image_styles,
        get_image_tags,
    },
};
use web_sys::{HtmlDialogElement, ShadowRoot};

impl Component<ImageTable> for Rc<ImageTable> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("admin-table-image", {
            .child(html!("input", {
                .prop("type", "checkbox")
                .prop("slot", "search")
                .prop_signal("indeterminate", state.image_len_selected_len_signal().map(|(image_len, selected_len)| {
                    selected_len > 0 && selected_len < image_len
                }))
                .prop_signal("checked", state.image_len_selected_len_signal().map(|(image_len, selected_len)| {
                    selected_len == image_len
                }))
                .event(clone!(state => move |_: events::Change| {
                    let images = state.images.lock_ref();
                    let mut selected = state.selected_images.lock_mut();
                    if selected.len() < images.len() {
                        for image in images.iter() {
                            selected.insert(image.id);
                        }
                    } else {
                        selected.clear();
                    }
                }))
            }))
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.search_images(e.query());
                }))
            }))
            .child(html!("button-rect", {
                .prop("slot", "controls")
                .prop("color", "blue")
                .prop("size", "small")
                .text("Mass edit")
                .prop_signal("disabled", state.selected_images.signal_ref(|selected_images| {
                    selected_images.is_empty()
                }))
                .event(clone!(state => move |_: events::Click| {
                    state.mass_editing.set(true);
                }))
            }))
            .child(html!("table-pagination-jig", {
                .prop("slot", "controls")
                .child(html!("fa-button", {
                    .prop("slot", "back")
                    .prop("title", "Previous")
                    .prop("icon", "fa-solid fa-chevron-left")
                    .prop_signal("disabled", state.active_page.signal().map(|active_page| {
                        active_page == 0
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        state.clear_selected();
                        let active_page = state.active_page.get();
                        state.go_to_page(active_page - 1);
                    }))
                }))
                .child(html!("fa-button", {
                    .prop("slot", "next")
                    .prop("title", "Next")
                    .prop("icon", "fa-solid fa-chevron-right")
                    .prop_signal("disabled", map_ref! {
                        let total_pages = state.total_pages.signal(),
                        let active_page = state.active_page.signal() => {
                            match total_pages {
                                None => true,
                                Some(total_pages) => {
                                    // active_page is 0 indexed in the code side, so need to add 1 for display
                                    *active_page == total_pages - 1
                                }
                            }
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        state.clear_selected();
                        let active_page = state.active_page.get();
                        state.go_to_page(active_page + 1);
                    }))
                }))
                .child_signal(state.total_pages.signal().map(clone!(state => move |total_pages| {
                    total_pages.map(|total_pages| {
                        html!("input-select", {
                            .prop_signal("value", state.active_page.signal().map(|active_page| {
                                format!("{}", active_page + 1)
                            }))
                            .children((0..total_pages).map(|page| {
                                html!("input-select-option", {
                                    .text(&format!("{}", page + 1).to_string())
                                    .prop_signal("selected", state.active_page.signal().map(clone!(page => move |active_page| {
                                        page == active_page
                                    })))
                                    .event(clone!(state, page => move |evt: events::CustomSelectedChange| {
                                        if evt.selected() {
                                            state.clear_selected();
                                            state.go_to_page(page);
                                        }
                                    }))
                                })
                            }))
                        })
                    })
                })))
            }))
            .children_signal_vec(state.images.signal_vec_cloned().map(clone!(state => move |image: Rc<EditableImage>| {
                html!("admin-table-line-image", {
                    .children(&mut [
                        html!("span", {
                            .child(html!("input-checkbox", {
                                .prop_signal("checked", state.selected_images.signal_ref(clone!(image => move |selected_images| {
                                    selected_images.contains(&image.id)
                                })))
                                .event(clone!(state, image => move |evt: events::Click| {
                                    state.checkbox_change(&image.id, evt.shift_key());
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("img-wrapper")
                            .child(html!("img-ji", {
                                .prop("size", "thumb")
                                .prop("lib", MediaLibrary::Global.to_str())
                                .prop("id", image.id.0.to_string())
                            }))
                        }),
                        html!("a", {
                            .text_signal(image.name.signal_cloned())
                            .prop("href", format!("/admin/image-meta/{}/false", image.id.0.to_string()))
                            // .event(clone!(state => move |_: events::Click| {
                            //     state.navigate_to(route);
                            // }))
                        }),
                        html!("span", {
                            .text_signal(image.description.signal_cloned())
                        }),
                        html!("span", {
                            .children_signal_vec(image.styles.signal_cloned().map(clone!(state => move |styles_hash| {
                                styles_hash.into_iter().map(|style_id| {
                                    html!("span", {
                                        .text_signal(state.styles_label(style_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                        html!("span", {
                            .children_signal_vec(image.tags.signal_cloned().map(clone!(state => move |tags_hash| {
                                tags_hash.into_iter().map(|tag_id| {
                                    html!("span", {
                                        .text_signal(state.tags_label(tag_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                        html!("span", {
                            .children_signal_vec(image.age_ranges.signal_cloned().map(clone!(state => move |ages_hash| {
                                ages_hash.into_iter().map(|age_id| {
                                    html!("span", {
                                        .text_signal(state.age_label(age_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                        html!("span", {
                            .children_signal_vec(image.affiliations.signal_cloned().map(clone!(state => move |affiliations_hash| {
                                affiliations_hash.into_iter().map(|affiliation_id| {
                                    html!("span", {
                                        .text_signal(state.affiliation_label(affiliation_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                        html!("span", {
                            .children_signal_vec(image.categories.signal_cloned().map(clone!(state => move |categories_hash| {
                                categories_hash.into_iter().map(|categories_id| {
                                    html!("span", {
                                        .text_signal(state.categories_label(categories_id))
                                    })
                                }).collect()
                            })).to_signal_vec())
                        }),
                    ])
                })
            })))
            .child_signal(state.mass_editing.signal().map(clone!(state => move |mass_editing| {
                mass_editing.then(clone!(state => move || {
                    state.render_mass_editing()
                }))
            })))
        }))
    }
}

impl ImageTable {
    fn image_len_selected_len_signal(&self) -> impl Signal<Item = (usize, usize)> {
        map_ref! {
            let image_len = self.images.signal_vec_cloned().len(),
            let selected_len = self.selected_images.signal_ref(|selected_images| selected_images.len()) => move {
                (*image_len, *selected_len)
            }
        }
    }

    fn render_mass_editing(self: &Rc<Self>) -> Dom {
        html!("dialog" => HtmlDialogElement, {
            .after_inserted(move |dialog| {
                let _ = dialog.show_modal();
            })
            .child(MassEditing::new(self).render())
        })
    }

    fn styles_label(self: &Rc<Self>, style_id: ImageStyleId) -> impl Signal<Item = String> {
        from_future(get_image_styles())
            .map(|x| x.unwrap_or_default())
            .map(move |styles| {
                let style = styles.iter().find(|style| style.id == style_id);
                match style {
                    Some(style) => style.display_name.clone(),
                    None => "-".to_string(),
                }
            })
    }

    fn tags_label(self: &Rc<Self>, tag_id: ImageTagIndex) -> impl Signal<Item = String> {
        from_future(get_image_tags())
            .map(|x| x.unwrap_or_default())
            .map(move |tags| {
                let tag = tags.iter().find(|tag| tag.index == tag_id);
                match tag {
                    Some(tag) => tag.display_name.clone(),
                    None => "-".to_string(),
                }
            })
    }

    fn age_label(self: &Rc<Self>, age_id: AgeRangeId) -> impl Signal<Item = String> {
        from_future(get_age_ranges())
            .map(|x| x.unwrap_or_default())
            .map(move |ages| {
                let age = ages.iter().find(|age| age.id == age_id);
                match age {
                    Some(age) => age.display_name.clone(),
                    None => "-".to_string(),
                }
            })
    }

    fn affiliation_label(
        self: &Rc<Self>,
        affiliation_id: AffiliationId,
    ) -> impl Signal<Item = String> {
        from_future(get_affiliations())
            .map(|x| x.unwrap_or_default())
            .map(move |affiliations| {
                let affiliation = affiliations
                    .iter()
                    .find(|affiliation| affiliation.id == affiliation_id);
                match affiliation {
                    Some(affiliation) => affiliation.display_name.clone(),
                    None => "-".to_string(),
                }
            })
    }

    fn categories_label(self: &Rc<Self>, category_id: CategoryId) -> impl Signal<Item = String> {
        from_future(get_category_label_lookup())
            .map(|x| x.unwrap_or_default())
            .map(move |categories| match categories.get(&category_id) {
                Some(category) => category.to_owned(),
                None => "-".to_string(),
            })
    }
}
