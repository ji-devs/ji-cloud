use dominator::{html, Dom, clone, with_node};
use std::rc::Rc;
use std::cell::RefCell;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use js_sys::Reflect;
use futures_signals::{
    map_ref,
    signal::{self, Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use dominator_helpers::signals::EitherSignal;
use components::image::search::types::*;
use components::tooltip::state::{State as TooltipState, TooltipData, TooltipConfirm, MoveStrategy, Placement};
use super::state::*;
use crate::base::state::Card;
use shared::domain::jig::module::body::memory::{Mode, Step};
use components::module::edit::prelude::*;

pub fn render(state:Rc<MainCard>) -> Dom {
    html!("main-card", {
        .property("slot", state.side.slot_name())
        .property("flippable", state.step == Step::Two)
        .property("editing", state.step == Step::One)
        .property_signal("dragOver", state.editing_active.signal())
        .property_signal("theme", state.base.theme_id_str_signal())

        .event(clone!(state => move |evt:events::Click| {
            if let Some(input_ref) = state.input_ref.borrow().as_ref() {
                Reflect::set(input_ref, &JsValue::from_str("editing"), &JsValue::from_bool(true));
                state.editing_active.set_neq(true);
            }

            if let Some(on_click) = state.callbacks.on_click.as_ref() {
                (on_click) ();
            }
        }))
        .child_signal({
            match &state.card {
                Card::Text(data) => {
                    EitherSignal::Left(signal::always(Some(
                        html!("input-textarea-content", {
                            .property_signal("value", data.signal_cloned())
                            .property_signal("fontSize", {
                                let sig = map_ref!{
                                    let value = data.signal_cloned(),
                                    let theme_id = state.base.theme_id.signal()
                                        => {
                                            (value.len(), *theme_id)
                                        }
                                };

                                sig.map(|(len, theme_id)| {
                                    let font_size = app_memory_common::lookup::get_card_font_size(len, theme_id);
                                    format!("{}px", font_size)
                                })
                            })
                            .property_signal("fontFamily", state.base.theme_id.signal().map(clone!(state => move |theme_id| {
                                let font_family = app_memory_common::lookup::get_card_font_family(theme_id, state.base.mode.into(), state.side.into());
                                theme_id.css_var_font_family(font_family)
                            })))
                            .property_signal("color", state.base.theme_id.signal().map(|theme_id| {
                                theme_id.css_var_color(1)
                            }))
                            .property("clickMode", "none")
                            .property("constrainWidth", crate::config::CARD_TEXT_LIMIT_WIDTH)
                            .property("constrainHeight", crate::config::CARD_TEXT_LIMIT_HEIGHT)
                            .event(clone!(state => move |evt:events::CustomInput| {
                                let index = state.index.get().unwrap_or_default();
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
                            .event(clone!(state, data => move |evt:events::Reset| {
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
                    )))
                },
                Card::Image(image) => {
                    EitherSignal::Right(image.signal_cloned().map(clone!(state => move |image| {
                            Some(match image {
                                None => {
                                    html!("img-ui", {
                                        .property("path", "core/_common/image-empty.svg")
                                        .event_preventable(clone!(state => move |evt:events::DragOver| {
                                            if let Some(data_transfer) = evt.data_transfer() {
                                                if data_transfer.types().index_of(&JsValue::from_str(IMAGE_SEARCH_DATA_TRANSFER), 0) != -1 {
                                                    evt.prevent_default();
                                                    state.editing_active.set_neq(true);
                                                }
                                            }

                                        }))
                                        .event(clone!(state => move |evt:events::DragLeave| {
                                            state.editing_active.set_neq(false);
                                        }))
                                        .event(clone!(state => move |evt:events::Drop| {
                                            if let Some(data_transfer) = evt.data_transfer() {
                                                if let Some(data) = data_transfer.get_data(IMAGE_SEARCH_DATA_TRANSFER).ok() { 
                                                    let data:ImageDataTransfer = serde_json::from_str(&data).unwrap_ji();
                                                    let index = state.index.get().unwrap_or_default();
                                                    state.replace_card_image(index, state.side, data.image);
                                                }
                                            }
                                            state.editing_active.set_neq(false);
                                        }))
                                    })
                                },
                                Some(image) => {
                                    html!("img-ji", {
                                        .property("size", "full")
                                        .property("id", image.id.0.to_string())
                                        .property("lib", image.lib.to_str())
                                    })
                                }
                            })
                    })))
                },
                _ => unimplemented!("no audio cards")
            }
        })
    })
}
