use dominator::{clone, html, with_node, Dom, EventOptions};
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use utils::prelude::*;
use wasm_bindgen::prelude::*;

use super::state::*;
use crate::{
    audio::mixer::AUDIO_MIXER,
    image::search::types::*,
    module::_groups::cards::{
        edit::{config, state::*},
        lookup::{self, Side},
    },
    overlay::handle::OverlayHandle,
};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::module::body::{
    ModeExt,
    _groups::cards::{Mode, Step},
};

const STR_CONFIRM_TITLE: &str = "Warning";

const STR_REMOVE_CONTENT_IMAGE: &str = "Are you sure you want to remove this image?";
const STR_REMOVE_CONTENT_AUDIO: &str = "Are you sure you want to remove this audio clip?";
const STR_REMOVE_CONFIRM: &str = "Yes, remove";
const STR_REMOVE_CANCEL: &str = "Don't remove";

const STR_DELETE_CONTENT_PAIR: &str = "Are you sure you want to delete this pair?";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<MainCard<RawData, E>>) -> Dom {
    let longest_card_text_signal = map_ref! {
        let pairs = state.base.pairs.signal_vec_cloned().to_signal_cloned()
            => {
                pairs.iter().fold(0, |acc, pair| {
                    let (a, b) = (&pair.0, &pair.1);
                    let longest_current = match (&a.card_content, &b.card_content) {
                        (CardContent::Text(a), CardContent::Text(b)) => {
                            a.get_cloned().graphemes(true).count().max(b.get_cloned().graphemes(true).count())
                        }
                        (CardContent::Text(a), _) => {
                            a.get_cloned().graphemes(true).count()
                        }
                        (_, CardContent::Text(b)) => {
                            b.get_cloned().graphemes(true).count()
                        }
                        _ => 0,
                    };

                    acc.max(longest_current)
                })
            }
    };

    html!("main-card", {
        .child_signal(state.confirm_action.signal_cloned().map(clone!(state => move |confirm_action| {
            confirm_action.map(|confirm_action| html!("empty-fragment", {
                    // The empty-fragment is required so that we can render the overly inside
                    // a signal, but adding the fragment upsets the layout of the cards because of
                    // their positioning. Setting it's display to none resolves the layout.
                    .style("display", "none")
                    .apply(OverlayHandle::lifecycle(clone!(state => move || {
                        let confirm_action = confirm_action.clone();
                        html!("modal-confirm", {
                            .prop("dangerous", true)
                            .prop("title", confirm_action.title)
                            .prop("content", confirm_action.content)
                            .prop("cancel_text", confirm_action.cancel)
                            .prop("confirm_text", confirm_action.confirm)
                            .prop("confirmIcon", "core/menus/delete-white.svg")
                            .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_action.set(None)))
                            .event(clone!(state => move |_evt: events::CustomConfirm| {
                                state.confirm_action.set(None);
                                (confirm_action.handler)();
                            }))
                        })
                    })))
                }))
        })))
        .prop("slot", state.side.as_str_id())
        .prop("side", state.side.as_str_id())
        .prop("flippable", state.step == Step::Two)
        .prop("editing", state.step == Step::One)
        .prop_signal("selected", state.base.selected_pair.signal_cloned().map(clone!(state => move |selected| {
            selected.map_or(false, |(idx, side)| {
                let correct_idx = state.index.get().unwrap_or_default() == idx;

                let correct_side = match side {
                    SelectedSide::One(side) => side == state.side,
                    SelectedSide::Both => true,
                };

                correct_idx && correct_side
            })
        })))
        .prop_signal("dragOver", state.editing_active.signal())
        .prop_signal("theme", state.base.theme_id_str_signal())
        .prop("mode", state.base.mode.as_str_id())
        .apply_if(state.card.audio.is_some(), |dom| {
            dom.child(html!("button-icon", {
                .prop("slot", "audio")
                .prop("icon", "audio")
                .event(clone!(state => move |_evt:events::Click| {
                    let audio = state.card.audio.as_ref().unwrap_ji();
                    AUDIO_MIXER.with(|mixer| {
                        mixer.play_oneshot(audio.into())
                    });
                }))
            }))
        })
        .apply_if(state.step == Step::One, |dom| {
            dom.child(html!("menu-kebab", {
                .prop("slot", "menu")
                .prop("positioningEnabled", false)
                .prop_signal("customContainer", state.menu_container_elem.signal_cloned())
                .prop_signal("visible", state.menu_open.signal_cloned())
                .with_node!(button_elem => {
                    .child_signal(state.menu_open.signal_cloned().map(clone!(state => move |is_open| {
                        if is_open {
                            Some(html!("empty-fragment", {
                                .apply(OverlayHandle::lifecycle(clone!(state, button_elem => move || {
                                    html!("menu-items", {
                                        .prop("target", button_elem.clone())
                                        .apply_if(matches!(state.card.card_content, CardContent::Text(_)), clone!(state => move |dom| {
                                            dom.child(html!("menu-line", {
                                                .prop("icon", "text")
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
                                                image.map(|_| html!("menu-line", {
                                                            .prop("icon", "image")
                                                            .prop("customLabel", "Remove image")
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
                                            })))
                                        }))
                                        .apply_if(state.card.audio.is_some(), clone!(state => move |dom| {
                                            dom.child(html!("menu-line", {
                                                .prop("icon", "record-sound")
                                                .prop("customLabel", "Remove audio")
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
                                            .prop("icon", "delete")
                                            .prop("customLabel", "Delete pair")
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
                        .prop_signal("value", data.signal_cloned())
                        .prop_signal("fontSize", longest_card_text_signal.map(|length| {
                            lookup::get_card_font_size(length, None)
                        }))
                        .prop_signal("editing", state.editing_active.signal_cloned())
                        .prop("clickMode", "none")
                        .prop("constrainWidth", config::CARD_TEXT_LIMIT_WIDTH)
                        .prop("constrainHeight", config::CARD_TEXT_LIMIT_HEIGHT)
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
                                        .prop("path", "core/_common/image-empty.svg")
                                    })
                                },
                                Some(image) => {
                                    html!("img-ji", {
                                        // would like to get rid if the styles here
                                        .style("height", "148px")
                                        .style("width", "148px")
                                        .style("object-fit", "contain")
                                        .prop("size", "full")
                                        .prop("id", image.id.0.to_string())
                                        .prop("lib", image.lib.to_str())
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
