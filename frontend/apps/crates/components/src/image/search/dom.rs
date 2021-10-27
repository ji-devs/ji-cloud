use crate::image::search::state::{ImageSearchCheckboxKind, NextPage, SearchMode};

use super::{
    actions,
    state::State,
    types::*,
};
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::{MutableVec, SignalVec, SignalVecExt}};
use shared::domain::{jig::module::body::Image, search::WebImageSearchItem};
use url::Url;
use std::{pin::Pin, rc::Rc, str::FromStr};
use utils::prelude::*;

const STR_SHOW_ONLY_BACKGROUNDS: &'static str = "Show only background";
const STR_DONT_INCLUDE_BACKGROUND: &'static str = "Do not include backgrounds";

pub fn render(state: Rc<State>, slot: Option<&str>) -> Dom {
    html!("empty-fragment", {
        .apply_if(slot.is_some(), move |dom| {
            dom.property("slot", slot.unwrap_ji())
        })
        .child_signal(state.init_loader.is_loading().map(clone!(state => move |init_loading| {
            if init_loading {
                Some(html!("p", {
                    .text("Loading...")
                }))
            } else {
                Some(render_loaded(state.clone()))
            }
        })))
    })
}

pub fn render_loaded(state: Rc<State>) -> Dom {
    actions::fetch_init_data(Rc::clone(&state));

    html!("image-select", {
        .property("label", "Select background")
        .property_signal("loading", state.loader.is_loading())
        .property_signal("recent", state.recent_list.signal_vec_cloned().len().map(|len| {
            len > 0
        }))
        .children(render_controls(state.clone()))
        .children_signal_vec(state.recent_list.signal_vec_cloned().map(clone!(state => move |image| {
            render_image(Rc::clone(&state), image, "recent")
        })))
        .children_signal_vec(state.search_mode.signal_cloned().switch_signal_vec(clone!(state => move |search_mode| {
            images_signal_vec(Rc::clone(&state), &search_mode)
        })))
        .apply_if(state.options.recent, |dom| {
            dom.child_signal(state.loader.is_loading().map(|is_loading| {
                match is_loading {
                    false => None,
                    true => {
                        Some(html!("p", {
                            .text("Loading..")
                        }))
                    },
                }
            }))
        })
        .event(clone!(state => move |_: events::ScrollEnd| {
            let search_mode = state.search_mode.lock_ref();
            if let SearchMode::Sticker(_) = &*search_mode {

                let next_page = *state.next_page.borrow();

                if let NextPage::Page(page) = next_page {
                    log::info!("Loading page {}", page);
                    actions::search(Rc::clone(&state), Some(page));
                } else {
                    log::info!("End, not loading more");
                };

            };
        }))
    })
}


fn images_signal_vec(state: Rc<State>, search_mode: &SearchMode) -> Pin<Box<dyn SignalVec<Item = Dom>>> {
    match search_mode {
        SearchMode::Sticker(images) => {
            let elements = images.signal_vec_cloned().map(clone!(state => move |image| {
                render_image(Rc::clone(&state), image, "images")
            }));

            Box::pin(elements)
        },
        SearchMode::Web(images) => {
            let elements = images.signal_vec_cloned().map(clone!(state => move |image| {
                render_web_image(Rc::clone(&state), image, "images")
            }));

            Box::pin(elements)
        },
    }
}


fn render_image(state: Rc<State>, image: Image, slot: &str) -> Dom {
    html!("img-ji", {
        .property("slot", slot)
        .property("size", "thumb")
        .property("lib", image.lib.to_str())
        .property("id", image.id.0.to_string())
        .event(clone!(state, image => move |_: events::Click| {
            state.set_selected(image.clone());
        }))
        .event(clone!(image => move |evt: events::DragStart| {
            if let Some(data_transfer) = evt.data_transfer() {
                let data = ImageDataTransfer::Image(image.clone());
                let json = serde_json::to_string(&data).unwrap_ji();
                let _ = data_transfer.set_data(IMAGE_SEARCH_DATA_TRANSFER, &json);
                data_transfer.set_drop_effect("all");
            } else {
                log::error!("no data transfer - use a real computer!!!");
            }
        }))
    })
}

fn render_web_image(state: Rc<State>, image: WebImageSearchItem, slot: &str) -> Dom {
    html!("img", {
        .property("slot", slot)
        .property("size", "thumb")
        .property("src", &image.thumbnail_url)
        .property("loading", "lazy")
        .event(clone!(state, image => move |_: events::Click| {
            actions::on_web_image_click(Rc::clone(&state), &image.url);
        }))
        .event(clone!(image => move |evt: events::DragStart| {
            if let Some(data_transfer) = evt.data_transfer() {
                // issue #1768 will resolve this
                let url = Url::from_str(&image.url).unwrap_ji();

                let data = ImageDataTransfer::Web(url);
                let json = serde_json::to_string(&data).unwrap_ji();
                let _ = data_transfer.set_data(IMAGE_SEARCH_DATA_TRANSFER, &json);
                data_transfer.set_drop_effect("all");
            } else {
                log::error!("no data transfer - use a real computer!!!");
            }
        }))
    })
}

fn render_controls(state: Rc<State>) -> Vec<Dom> {
    let options = &state.clone().options;
    let mut vec = Vec::new();

    if let Some(checkbox_kind) = &options.checkbox_kind {
        vec.push(html!("input-checkbox", {
            .property("label", {
                match checkbox_kind {
                    ImageSearchCheckboxKind::BackgroundLayer1Filter | ImageSearchCheckboxKind::BackgroundLayer2Filter => STR_SHOW_ONLY_BACKGROUNDS,
                    ImageSearchCheckboxKind::StickersFilter => STR_DONT_INCLUDE_BACKGROUND,
                }
            })
            .property("slot", "only-background-checkbox")
            .property("checked", true)
            .property_signal("disabled", state.search_mode.signal_ref(|search_mode| {
                search_mode.is_web()
            }))
            .event(clone!(state => move |evt: events::CustomToggle| {
                state.checkbox_checked.set(evt.value());
                actions::search(state.clone(), None);
            }))
        }));
    }

    if options.upload {
        vec.push(html!("image-search-upload", {
            .property("slot", "upload")
            .property("label", "Upload")
            .event(clone!(state => move |e: events::CustomFile| {
                let file = e.file();
                state.loader.load(clone!(state => async move {
                    actions::upload_file(state.clone(), file).await;
                }));
            }))
        }));
    }

    if options.filters {
        vec.push(render_filters(state.clone()));
    }

    vec.push(html!("input-search", {
        .property("placeholder", "Search")
        .property("slot", "search-input")
        .event(clone!(state => move |e: events::CustomSearch| {
            state.query.set(e.query());
            actions::search(state.clone(), None);
        }))
    }));

    vec
}

fn render_filters(state: Rc<State>) -> Dom {
    html!("image-search-filters", {
        .property("slot", "filters")
        .children(&mut [
            html!("label", {
                .property("slot", "source-options")
                .child(html!("input", {
                    .property("type", "radio")
                    .property("name", "type")
                    .property("value", "web")
                    .property_signal("checked", state.search_mode.signal_ref(|search_mode| {
                        matches!(search_mode, &SearchMode::Sticker(_))
                    }))
                    .event(clone!(state => move |_: events::Change| {
                        state.search_mode.set(SearchMode::Sticker(Rc::new(MutableVec::new())));
                        actions::search(Rc::clone(&state), None);
                    }))
                }))
                .text("Stickers")
            }),
            html!("label", {
                .property("slot", "source-options")
                .child(html!("input", {
                    .property("type", "radio")
                    .property("name", "type")
                    .property("value", "stickers")
                    .property_signal("checked", state.search_mode.signal_ref(|search_mode| {
                        matches!(search_mode, &SearchMode::Web(_))
                    }))
                    .event(clone!(state => move |_: events::Change| {
                        state.search_mode.set(SearchMode::Web(Rc::new(MutableVec::new())));
                        actions::search(Rc::clone(&state), None);
                    }))
                }))
                .text("Web")
            }),
        ])
        .children(
            state
                .styles
                .borrow()
                .as_ref()
                .unwrap_ji()
                .iter()
                .map(clone!(state => move |style| {
                    html!("image-search-style-option", {
                        .property("slot", "style-options")
                        .property("label", &style.display_name)
                        .apply(|dom| {
                            let style_id = style.id.clone();
                            dom.event(clone!(state => move |e: events::CustomToggle| {
                                match e.value() {
                                    true => state.selected_styles.as_ref().borrow_mut().insert(style_id),
                                    false => state.selected_styles.as_ref().borrow_mut().remove(&style_id),
                                };
                                actions::search(state.clone(), None);
                            }))
                        })

                    })
                }))
        )
    })
}
