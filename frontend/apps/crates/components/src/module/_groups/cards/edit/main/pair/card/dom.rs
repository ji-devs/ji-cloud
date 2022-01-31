use dominator::{clone, html, Dom, EventOptions, with_node};
use web_sys::HtmlElement;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

use utils::prelude::*;
use wasm_bindgen::prelude::*;

use super::state::*;
use crate::{
    image::search::types::*,
    module::_groups::cards::{
        edit::{config, state::*},
        lookup::{self, Side},
    }, overlay::handle::OverlayHandle,
};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::{
    ModeExt,
    _groups::cards::{Mode, Step},
};

const STR_CONFIRM_TITLE: &'static str = "Warning";

const STR_REMOVE_CONTENT_IMAGE: &'static str = "Are you sure you want to remove this image?";
const STR_REMOVE_CONTENT_AUDIO: &'static str = "Are you sure you want to remove this audio clip?";
const STR_REMOVE_CONFIRM: &'static str = "Remove";
const STR_REMOVE_CANCEL: &'static str = "Don't remove";

const STR_DELETE_CONTENT_PAIR: &'static str = "Are you sure you want to delete this pair?";
const STR_DELETE_CONFIRM: &'static str = "Delete Pair";
const STR_DELETE_CANCEL: &'static str = "Don't delete";

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<MainCard<RawData, E>>) -> Dom {
    html!("main-card", {
        .child_signal(state.confirm_action.signal_cloned().map(clone!(state => move |confirm_action| {
            if let Some(confirm_action) = confirm_action {
                Some(html!("empty-fragment", {
                    // The empty-fragment is required so that we can render the overly inside
                    // a signal, but adding the fragment upsets the layout of the cards because of
                    // their positioning. Setting it's display to none resolves the layout.
                    .style("display", "none")
                    .apply(OverlayHandle::lifecycle(clone!(state => move || {
                        let confirm_action = confirm_action.clone();
                        html!("modal-confirm", {
                            .property("dangerous", true)
                            .property("title", confirm_action.title)
                            .property("content", confirm_action.content)
                            .property("cancel_text", confirm_action.cancel)
                            .property("confirm_text", confirm_action.confirm)
                            .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_action.set(None)))
                            .event(clone!(state => move |_evt: events::CustomConfirm| {
                                state.confirm_action.set(None);
                                (confirm_action.handler)();
                            }))
                })
                    })))
                }))
            } else {
                None
            }
        })))
        .property("slot", state.side.as_str_id())
        .property("side", state.side.as_str_id())
        .property("flippable", state.step == Step::Two)
        .property("editing", state.step == Step::One)
        .property_signal("selected", state.base.selected_pair.signal_cloned().map(clone!(state => move |selected| {
            selected.map_or(false, |(idx, side)| {
                let correct_idx = state.index.get().unwrap_or_default() == idx;

                let correct_side = match side {
                    SelectedSide::One(side) => side == state.side,
                    SelectedSide::Both => true,
                };

                correct_idx && correct_side
            })
        })))
        .property_signal("dragOver", state.editing_active.signal())
        .property_signal("theme", state.base.theme_id_str_signal())
        .property("mode", state.base.mode.as_str_id())
        .apply_if(state.card.audio.is_some(), |dom| {
            dom.child(html!("button-icon", {
                .property("slot", "audio")
                .property("icon", "audio")
            }))
        })
        .apply_if(state.step == Step::One, |dom| {
            dom.child(html!("menu-kebab", {
                .property("slot", "menu")
                .property("positioningEnabled", false)
                .property_signal("customContainer", state.menu_container_elem.signal_cloned())
                .property_signal("visible", state.menu_open.signal_cloned())
                .with_node!(button_elem => {
                    .child_signal(state.menu_open.signal_cloned().map(clone!(state => move |is_open| {
                        if is_open {
                            Some(html!("empty-fragment", {
                                .apply(OverlayHandle::lifecycle(clone!(state, button_elem => move || {
                                    html!("menu-items", {
                                        .property("target", button_elem.clone())
                                        .apply_if(matches!(state.card.card_content, CardContent::Text(_)), clone!(state => move |dom| {
                                            dom.child(html!("menu-line", {
                                                .property("icon", "text")
                                                .event(clone!(state => move |_evt:events::Click| {
                                                    state.close_menu();
                                                    state.editing_active.set_neq(true);

                                                    if let Some(on_click) = state.callbacks.on_click.as_ref() {
                                                        (on_click) ();
                                                    }
                                                }))
                                            }))
                                        }))
                                        .apply_if(matches!(state.card.card_content, CardContent::Image(_)), clone!(state => move |dom| {
                                            dom.child_signal(state.card.as_image_mutable().signal_cloned().map(clone!(state => move |image| {
                                                match image {
                                                    Some(_) => {
                                                        Some(html!("menu-line", {
                                                            .property("icon", "image")
                                                            .property("customLabel", "Remove image")
                                                            .event(clone!(state => move |_evt:events::Click| {
                                                                state.close_menu();
                                                                state.confirm_action.set(Some(Rc::new(ModalAction::new(
                                                                    STR_CONFIRM_TITLE,
                                                                    STR_REMOVE_CONTENT_IMAGE,
                                                                    STR_REMOVE_CONFIRM,
                                                                    STR_REMOVE_CANCEL,
                                                                    Rc::new(clone!(state => move || {
                                                                        let index = state.index.get().unwrap_or_default();
                                                                        state.remove_card_image(index, state.side);
                                                                    }))
                                                                ))))
                                                            }))
                                                        }))
                                                    },
                                                    None => None,
                                                }
                                            })))
                                        }))
                                        .apply_if(state.card.audio.is_some(), clone!(state => move |dom| {
                                            dom.child(html!("menu-line", {
                                                .property("icon", "record-sound")
                                                .property("customLabel", "Remove audio")
                                                .event(clone!(state => move |_evt:events::Click| {
                                                    state.close_menu();
                                                    state.confirm_action.set(Some(Rc::new(ModalAction::new(
                                                        STR_CONFIRM_TITLE,
                                                        STR_REMOVE_CONTENT_AUDIO,
                                                        STR_REMOVE_CONFIRM,
                                                        STR_REMOVE_CANCEL,
                                                        Rc::new(clone!(state => move || {
                                                            let index = state.index.get().unwrap_or_default();
                                                            state.base.replace_pair(index, |mut pair| {
                                                                match state.side {
                                                                    Side::Left => { pair.0.audio = None },
                                                                    Side::Right => { pair.1.audio = None },
                                                                }

                                                                pair
                                                            })
                                                        }))
                                                    ))))
                                                }))
                                            }))
                                        }))
                                        .child(html!("menu-line", {
                                            .property("icon", "delete")
                                            .property("customLabel", "Delete pair")
                                            .event(clone!(state => move |_evt:events::Click| {
                                                state.close_menu();
                                                state.confirm_action.set(Some(Rc::new(ModalAction::new(
                                                    STR_CONFIRM_TITLE,
                                                    STR_DELETE_CONTENT_PAIR,
                                                    STR_DELETE_CONFIRM,
                                                    STR_DELETE_CANCEL,
                                                    Rc::new(clone!(state => move || {
                                                        state.delete_pair(state.index.get().unwrap_or_default());
                                                    }))
                                                ))))
                                            }))
                                        }))
                                        .after_inserted(clone!(state => move |elem| {
                                            state.menu_container_elem.set(Some(elem));
                                        }))
                                    })
                                })))
                            }))
                        } else {
                            None
                        }
                    })))
                })
                .event_with_options(&EventOptions::bubbles(), clone!(state => move |_e: events::Open| {
                    state.menu_open.set_neq(true);
                }))
                .event_with_options(&EventOptions::bubbles(), clone!(state => move |_e: events::Close| {
                    state.close_menu();
                }))
            }))
        })
        .event(clone!(state => move |_evt: events::MouseEnter| {
            state.is_hovering.set_neq(true);
        }))
        .event(clone!(state => move |_evt: events::MouseLeave| {
            state.is_hovering.set_neq(false);
        }))
        .event(clone!(state => move |evt: events::Click| {
            // [Ty] Prevents clicking on the menu kebab from selecting the card. The click event
            // fires here first and then in the menu-kebab element, so stopPropagation fails.
            let should_toggle = match evt.target() {
                Some(target) => {
                    let target: JsValue = target.into();
                    let element: HtmlElement = target.into();
                    let tag_name = element.tag_name().to_lowercase();
                    let tag_name = tag_name.as_str();
                    tag_name != "menu-kebab" && tag_name != "button-icon"
                },
                _ => true
            };

            if should_toggle {
                state.toggle_selection();
            }
        }))
        .child({
            match &state.card.card_content {
                CardContent::Text(data) => {
                    html!("input-textarea-content", {
                        .property_signal("value", data.signal_cloned())
                        .property_signal("fontSize", data.signal_cloned().map(|value| {
                            lookup::get_card_font_size(&value, None)
                        }))
                        .property_signal("editing", state.editing_active.signal_cloned())
                        .property("clickMode", "none")
                        .property("constrainWidth", config::CARD_TEXT_LIMIT_WIDTH)
                        .property("constrainHeight", config::CARD_TEXT_LIMIT_HEIGHT)
                        .event(clone!(state => move |evt:events::CustomInput| {
                            let _index = state.index.get().unwrap_or_default();
                            let value = evt.value();

                            if state.base.mode == Mode::Duplicate {
                                state.other.as_text_mutable().set_neq(value);
                            }
                        }))
                        .event(clone!(state => move |evt:events::CustomChange| {
                            let index = state.index.get().unwrap_or_default();
                            let value = evt.value();
                            state.replace_card_text(index, state.side, value);
                        }))
                        .event(clone!(state => move |evt:events::CustomToggle| {
                            state.editing_active.set_neq(evt.value());
                        }))
                        .event(clone!(state, data => move |_evt:events::Reset| {
                            //Just need to change the linked pair
                            //without affecting history
                            if state.base.mode == Mode::Duplicate {
                                //other.as_text_mutable().set_neq(original_data.clone());
                                state.other.as_text_mutable().set_neq(data.get_cloned());
                            }

                        }))
                        .after_inserted(clone!(state => move |dom| {
                            *state.input_ref.borrow_mut() = Some(dom);
                        }))
                    })
                },
                CardContent::Image(image) => {
                    html!("div", {
                        .event_with_options(
                            &EventOptions::preventable(),
                            clone!(state => move |evt:events::DragOver| {
                                if let Some(data_transfer) = evt.data_transfer() {
                                    if data_transfer.types().index_of(&JsValue::from_str(IMAGE_SEARCH_DATA_TRANSFER), 0) != -1 {
                                        evt.prevent_default();
                                        state.editing_active.set_neq(true);
                                    }
                                }
                            })
                        )
                        .event(clone!(state => move |_evt:events::DragLeave| {
                            state.editing_active.set_neq(false);
                        }))
                        .event_with_options(
                            &EventOptions::preventable(),
                            clone!(state => move |evt:events::Drop| {
                                evt.prevent_default();
                                if let Some(data_transfer) = evt.data_transfer() {
                                    if let Ok(data) = data_transfer.get_data(IMAGE_SEARCH_DATA_TRANSFER) {
                                        let data:ImageDataTransfer = serde_json::from_str(&data).unwrap_ji();
                                        spawn_local(clone!(state => async move {
                                            let image = data.to_image().await;
                                            let index = state.index.get().unwrap_or_default();
                                            state.replace_card_image(index, state.side, image);
                                        }));
                                    }
                                }
                                state.editing_active.set_neq(false);
                            })
                        )
                        .child_signal(image.signal_cloned().map(|image| {
                            Some(match image {
                                None => {
                                    html!("img-ui", {
                                        .property("path", "core/_common/image-empty.svg")
                                    })
                                },
                                Some(image) => {
                                    html!("img-ji", {
                                        // would like to get rid if the styles here
                                        .style("height", "148px")
                                        .style("width", "148px")
                                        .style("object-fit", "contain")
                                        .property("size", "full")
                                        .property("id", image.id.0.to_string())
                                        .property("lib", image.lib.to_str())
                                    })
                                }
                            })
                        }))
                    })
                }
            }
        })
    })
}
