use dominator::{Dom, html, clone};
use utils::prelude::*;
use std::rc::Rc;
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use super::{
    state::{State, BACKGROUND_NAME},
    actions::{get_background_id, search, upload_file},
    types::*,
};
use shared::{domain::jig::module::body::Image, media::MediaLibrary};

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
    state.loader.load(clone!(state => async move {
        search(state);
    }));

    html!("image-select", {
        .property("label", "Select background")
        .children(render_controls(state.clone()))
        .children_signal_vec(state.image_list.signal_vec_cloned().map(clone!(state => move |image| {
            html!("img-ji", {
                .property("slot", "images")
                .property("size", "thumb")
                .property("id", image.id.0.to_string())
                .event(clone!(state, image => move |_: events::Click| {
                    //TODO - this should change if the library has changed
                    state.set_selected(Image { id: image.id, lib: MediaLibrary::Global});
                }))
                .event(clone!(image => move |evt: events::DragStart| {
                    if let Some(data_transfer) = evt.data_transfer() {
                        let data = ImageDataTransfer {
                            image: Image { 
                                id: image.id.clone(),
                                //TODO - this should change if the library has changed
                                lib: MediaLibrary::Global
                            }
                        };
                        let json = serde_json::to_string(&data).unwrap_ji();
                        let _ = data_transfer.set_data(IMAGE_SEARCH_DATA_TRANSFER, &json);
                        data_transfer.set_drop_effect("all");
                    } else {
                        log::error!("no data transfer - use a real computer!!!");
                    }
                }))
            })
        })))
        .child_signal(state.loader.is_loading().map(|is_loading| {
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
}

fn render_controls(state: Rc<State>) -> Vec<Dom> {
    let options = &state.clone().options;
    let mut vec = Vec::new();

    if options.background_only.is_some() {
        vec.push(html!("input-checkbox", {
            .property("label", "Show only background")
            .property("slot", "only-background-checkbox")
            .property("checked", options.background_only.unwrap())
            .event(clone!(state => move |e: events::CustomToggle| {
                let style_id = get_background_id(&state.styles.borrow().as_ref().unwrap_ji());
                match e.value() {
                    true => state.selected_styles.as_ref().borrow_mut().insert(style_id),
                    false => state.selected_styles.as_ref().borrow_mut().remove(&style_id),
                };
                search(state.clone());
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
                    upload_file(state.clone(), file).await;
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
            search(state.clone());
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
                    .property("checked", true)
                }))
                .text("Stickers")
            }),
            html!("label", {
                .property("slot", "source-options")
                .child(html!("input", {
                    .property("type", "radio")
                    .property("name", "type")
                    .property("value", "stikers")
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
                .filter(|style| style.display_name != BACKGROUND_NAME)
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
                                search(state.clone());
                            }))
                        })
                        
                    })
                }))
        )
    })
}
