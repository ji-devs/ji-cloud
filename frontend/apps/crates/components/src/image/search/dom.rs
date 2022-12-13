use crate::image::search::state::{ImageSearchKind, NextPage, SearchMode};

use super::{actions, state::State, types::*};
use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
use shared::domain::{
    module::body::Image,
    search::{ImageType, WebImageSearchItem},
};
use std::{pin::Pin, rc::Rc};
use strum::IntoEnumIterator;
use utils::prelude::*;

const STR_SHOW_ONLY_BACKGROUNDS: &str = "Only background";
const STR_DONT_INCLUDE_BACKGROUND: &str = "No backgrounds";
const STR_WEB: &str = "Web";
const STR_JIGZI: &str = "Jigzi";
const STR_SEARCH: &str = "Search";

pub fn render(state: Rc<State>, slot: Option<&str>) -> Dom {
    render_with_action(state, slot, None::<fn() -> Dom>)
}

pub fn render_with_action(
    state: Rc<State>,
    slot: Option<&str>,
    get_action: Option<impl Fn() -> Dom + 'static>,
) -> Dom {
    html!("empty-fragment", {
        .apply_if(slot.is_some(), move |dom| {
            dom.prop("slot", slot.unwrap_ji())
        })
        .child_signal(state.init_loader.is_loading().map(clone!(state => move |init_loading| {
            if init_loading {
                Some(html!("p", {
                    .text("Loading...")
                }))
            } else {
                let action = get_action.as_ref().map(|get_action| get_action());
                Some(render_loaded(state.clone(), action))
            }
        })))
    })
}

pub fn render_loaded(state: Rc<State>, action: Option<Dom>) -> Dom {
    actions::fetch_init_data(Rc::clone(&state));

    html!("image-select", {
        .prop("label", state.options.kind.label())
        .prop("imageMode", {
            match &state.options.kind {
                ImageSearchKind::Sticker => "image",
                _ => "background"
            }
        })
        .prop_signal("loading", state.loader.is_loading())
        .prop_signal("recent", state.recent_list.signal_vec_cloned().len().map(clone!(state => move |len| {
            state.recent && len > 0
        })))
        .apply_if(action.is_some(), |dom| {
            dom.child(html!("empty-fragment", {
                .prop("slot", "action")
                .child(action.unwrap_ji())
            }))
        })
        .children(render_controls(state.clone()))
        .apply_if(state.options.kind != ImageSearchKind::Sticker, clone!(state => move |dom| {
            dom.child(html!("img-ui", {
                .prop("slot", "images")
                .prop("path", {
                    match state.options.kind {
                        ImageSearchKind::Background => "module/_common/edit/widgets/sidebar/image-select/clear-image-background.svg",
                        ImageSearchKind::Overlay => "module/_common/edit/widgets/sidebar/image-select/clear-image-overlay.svg",
                        ImageSearchKind::Sticker => unreachable!(),
                    }
                })
                .event(clone!(state => move |_: events::Click| {
                    state.clear_selected();
                }))
            }))
        }))
        .children_signal_vec(state.recent_list.signal_vec_cloned().map(clone!(state => move |image| {
            render_image(Rc::clone(&state), image, "recent")
        })))
        .children_signal_vec(state.search_mode.signal_cloned().switch_signal_vec(clone!(state => move |search_mode| {
            images_signal_vec(Rc::clone(&state), &search_mode)
        })))
        .apply_if(state.recent, |dom| {
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

fn images_signal_vec(
    state: Rc<State>,
    search_mode: &SearchMode,
) -> Pin<Box<dyn SignalVec<Item = Dom>>> {
    match search_mode {
        SearchMode::Sticker(images) => {
            let elements = images
                .signal_vec_cloned()
                .map(clone!(state => move |image| {
                    render_image(Rc::clone(&state), image, "images")
                }));

            Box::pin(elements)
        }
        SearchMode::Web(images) => {
            let elements = images
                .signal_vec_cloned()
                .map(clone!(state => move |image| {
                    render_web_image(Rc::clone(&state), image, "images")
                }));

            Box::pin(elements)
        }
    }
}

fn render_image(state: Rc<State>, image: Image, slot: &str) -> Dom {
    html!("img-ji", {
        .prop("slot", slot)
        .prop("size", "thumb")
        .prop("lib", image.lib.to_str())
        .prop("id", image.id.0.to_string())
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
        .prop("slot", slot)
        .prop("size", "thumb")
        .prop("src", &image.thumbnail_url.to_string())
        .prop("loading", "lazy")
        .event(clone!(state, image => move |_: events::Click| {
            actions::on_web_image_click(Rc::clone(&state), image.url.clone());
        }))
        .event(clone!(image => move |evt: events::DragStart| {
            if let Some(data_transfer) = evt.data_transfer() {
                let data = ImageDataTransfer::Web(image.url.clone());
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
    let options = &state.options;
    let mut vec = Vec::new();

    match state.options.kind {
        ImageSearchKind::Overlay => {
            // overlay can only search in jigzi and doesn't have filters or upload
        }
        ImageSearchKind::Background | ImageSearchKind::Sticker => {
            vec.push(html!("label", {
                .prop("slot", "source-options")
                .child(html!("input", {
                    .prop("type", "radio")
                    .prop("name", "type")
                    .prop("value", "web")
                    .style("margin", "0")
                    .prop_signal("checked", state.search_mode.signal_ref(|search_mode| {
                        matches!(search_mode, &SearchMode::Sticker(_))
                    }))
                    .event(clone!(state => move |_: events::Change| {
                        state.search_mode.set(SearchMode::Sticker(Rc::new(MutableVec::new())));
                        actions::search(Rc::clone(&state), None);
                    }))
                }))
                .text(STR_JIGZI)
            }));
            vec.push(html!("label", {
                .prop("slot", "source-options")
                .child(html!("input", {
                    .prop("type", "radio")
                    .prop("name", "type")
                    .prop("value", "stickers")
                    .style("margin", "0")
                    .prop_signal("checked", state.search_mode.signal_ref(|search_mode| {
                        matches!(search_mode, &SearchMode::Web(_))
                    }))
                    .event(clone!(state => move |_: events::Change| {
                        state.search_mode.set(SearchMode::Web(Rc::new(MutableVec::new())));
                        actions::search(Rc::clone(&state), None);
                    }))
                }))
                .text(STR_WEB)
            }));

            if options.upload {
                vec.push(html!("image-search-upload", {
                    .prop("slot", "upload")
                    .prop("label", "Upload")
                    .event(clone!(state => move |e: events::CustomFile| {
                        let file = e.file();
                        state.loader.load(clone!(state => async move {
                            actions::upload_file(state.clone(), file).await;
                        }));
                    }))
                }));
            }

            if options.filters {
                vec.push(render_filters(&state));
            }
        }
    };

    vec.push(html!("input-search", {
        .prop_signal("placeholder", state.search_mode.signal_ref(|search_mode| {
            let s = match search_mode {
                SearchMode::Sticker(_) => STR_JIGZI,
                SearchMode::Web(_) => STR_WEB,
            };
            format!("{} {}", STR_SEARCH, s)
        }))
        .prop("slot", "search-input")
        .event(clone!(state => move |e: events::CustomSearch| {
            state.query.set(e.query());
            actions::search(state.clone(), None);
        }))
    }));

    vec
}

fn render_filters(state: &Rc<State>) -> Dom {
    html!("empty-fragment", {
        .prop("slot", "filters")
        .child_signal(state.search_mode.signal_cloned().map(clone!(state => move |search_mode| {
            Some(match search_mode {
                SearchMode::Sticker(_) => render_filters_sticker(&state),
                SearchMode::Web(_) => render_filters_web(&state),
            })
        })))
    })
}

fn render_filters_sticker(state: &Rc<State>) -> Dom {
    html!("image-search-filters", {
        .apply(|dom| {
            dom.child(html!("input-checkbox", {
                .prop("label", {
                    match &state.options.kind {
                        ImageSearchKind::Background | ImageSearchKind::Overlay => STR_SHOW_ONLY_BACKGROUNDS,
                        ImageSearchKind::Sticker => STR_DONT_INCLUDE_BACKGROUND,
                    }
                })
                .prop("slot", "background-checkbox")
                .event(clone!(state => move |evt: events::CustomToggle| {
                    state.checkbox_checked.set(evt.value());
                    actions::search(state.clone(), None);
                }))
            }))
        })

        .children(
            state
                .styles
                .borrow()
                .as_ref()
                .unwrap_ji()
                .iter()
                .map(clone!(state => move |style| {
                    html!("image-search-style-option", {
                        .prop("slot", "style-options")
                        .prop("label", &style.display_name)
                        .apply(|dom| {
                            let style_id = style.id;
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

fn render_filters_web(state: &Rc<State>) -> Dom {
    html!("image-search-filters", {
        .prop("slot", "filters")
        .children(
            ImageType::iter()
                .filter(|it| it != &ImageType::AnimatedGif)
                .map(clone!(state => move |image_type| {
                    html!("image-search-style-option", {
                        .prop("slot", "style-options")
                        .prop("label", image_type.to_str())
                        .prop_signal("selected", state.selected_image_type.signal().map(move |selected_image_type| {
                            match selected_image_type {
                                Some(selected) => selected == image_type,
                                None => false,
                            }
                        }))
                        .event(clone!(state => move |_: events::CustomToggle| {
                            let selected = state.selected_image_type.get();
                            let value = match selected {
                                Some(selected) => {
                                    if selected == image_type {
                                        None
                                    } else {
                                        Some(image_type)
                                    }
                                },
                                None => Some(image_type),
                            };
                            state.selected_image_type.set(value);
                            actions::search(state.clone(), None);
                        }))

                    })
                }))
        )
    })
}
