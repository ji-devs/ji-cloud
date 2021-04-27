use dominator::{html, Dom, clone, with_node};
use crate::data::{raw, state::*};
use std::rc::Rc;
use std::cell::RefCell;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use js_sys::Reflect;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::SignalVecExt,
};
use components::image_search::types::*;
use components::tooltip::types::*;

pub struct PairDom {}
impl PairDom {
    pub fn render(state:Rc<State>, mode: Mode, step: Step, index: ReadOnlyMutable<Option<usize>>, pair:(Card, Card)) -> Dom {

        let left = CardDom::render(state.clone(), mode, step, index.clone(), Side::Left, pair.0.clone(), pair.1.clone());
        let right = CardDom::render(state.clone(), mode, step, index.clone(), Side::Right, pair.1, pair.0);

        if step == Step::One {
            html!("main-card-pair", {
                .property("hoverable", true)
                .property_signal("hoverLock", state.overlay.tooltips.delete.signal_ref(|x| x.is_some()))
                .property_signal("index", index.signal().map(|x| {
                    JsValue::from_f64(x.unwrap_or_default() as f64)
                }))
                .child(left)
                .child(right)
                .child(html!("button-icon" => HtmlElement, {
                    .property("slot", "close")
                    .property("icon", "circle-x-blue")
                    .with_node!(elem => {
                        .event(move |evt:events::Click| {
                            state.overlay.tooltips.delete.set(Some(
                                TooltipData::Confirm(TooltipConfirm {
                                    elem: elem.clone(), 
                                    placement: Placement::Right, 
                                    move_strategy: MoveStrategy::Destroy,
                                    slot: None,
                                    max_width: Some(180.0),
                                    header: crate::strings::confirm::STR_DELETE_PAIR_HEADER.to_string(),
                                    confirm_label: crate::strings::confirm::STR_DELETE_PAIR_CONFIRM.to_string(),
                                    cancel_label: crate::strings::confirm::STR_DELETE_PAIR_CANCEL.to_string(),
                                    on_confirm: Rc::new(Box::new(clone!(state, index => move || {
                                        state.delete_pair(index.get().unwrap_or_default());
                                        state.overlay.tooltips.delete.set(None); 
                                    }))),
                                    on_cancel: Rc::new(Box::new(clone!(state, index => move || {
                                        state.overlay.tooltips.delete.set(None); 
                                    })))
                                })
                            ));
                        })
                    })
                }))
            })
        } else {
            html!("main-card-pair", {
                .property("hoverable", false)
                .property_signal("index", index.signal().map(|x| {
                    JsValue::from_f64(x.unwrap_or_default() as f64)
                }))
                .child(left)
                .child(right)
            })
        }
    }
}

struct CardDom {}

impl CardDom {
    pub fn render(state:Rc<State>, mode: Mode, step: Step, index: ReadOnlyMutable<Option<usize>>, side:Side, card: Card, other: Card) -> Dom {
        let input_ref:Rc<RefCell<Option<HtmlElement>>> = Rc::new(RefCell::new(None));

        let editing_active:Mutable<bool> = Mutable::new(false);

        let is_image = match card {
            Card::Image(_) => true,
            _ => false,
        };

        let mode = state.mode.get().unwrap_ji();
        html!("main-card", {
            .property("slot", side.slot_name())
            .property("flippable", step == Step::Two)
            .property("editing", step == Step::One)
            .property_signal("dragOver", editing_active.signal())
            .property_signal("theme", state.theme_id_str_signal())

            .event(clone!(state, input_ref, editing_active, is_image => move |evt:events::Click| {
                if let Some(input_ref) = input_ref.borrow().as_ref() {
                    Reflect::set(input_ref, &JsValue::from_str("editing"), &JsValue::from_bool(true));
                    editing_active.set_neq(true);
                }

                if is_image {
                    if let Some(cb) = state.image_card_click_callback.borrow().as_ref() {
                        (cb)();
                    }
                }
            }))
            .child({
                match card {
                    Card::Text(data) => {
                        html!("input-textarea-content", {
                            .property_signal("value", data.signal_cloned())
                            .property_signal("fontSize", {
                                let sig = map_ref!{
                                    let value = data.signal_cloned(),
                                    let theme = state.theme_id.signal_cloned()
                                        => {
                                            (value.len(), *theme)
                                        }
                                };

                                sig.map(|(len, theme_id)| {
                                    let font_size = app_memory_common::lookup::get_card_font_size(len, theme_id);
                                    format!("{}px", font_size)
                                })
                            })
                            .property_signal("fontFamily", state.theme_id.signal_cloned().map(clone!(side, mode => move |theme_id| {
                                let font_family = app_memory_common::lookup::get_card_font_family(theme_id, mode, side.into());
                                theme_id.css_var_font_family(font_family)
                            })))
                            .property_signal("color", state.theme_id.signal_cloned().map(|theme_id| {
                                theme_id.css_var_color(1)
                            }))
                            .property("clickMode", "none")
                            .property("constrainWidth", crate::config::CARD_TEXT_LIMIT_WIDTH)
                            .property("constrainHeight", crate::config::CARD_TEXT_LIMIT_HEIGHT)
                            .event(clone!(state, index, other => move |evt:events::CustomInput| {
                                let index = index.get().unwrap_or_default();
                                let value = evt.value();

                                if mode == Mode::Duplicate {
                                    other.as_text_mutable().set_neq(value);
                                }
                            }))
                            .event(clone!(state, index => move |evt:events::CustomChange| {
                                let index = index.get().unwrap_or_default();
                                let value = evt.value();
                                state.replace_card_text(index, side, value);
                            }))
                            .event(clone!(editing_active => move |evt:events::CustomToggle| {
                                editing_active.set_neq(evt.value());
                            }))
                            .event(clone!(state, other => move |evt:events::Reset| {
                                //Just need to change the linked pair
                                //without affecting history
                                if mode == Mode::Duplicate {
                                    //other.as_text_mutable().set_neq(original_data.clone());
                                    other.as_text_mutable().set_neq(data.get_cloned());
                                }

                            }))
                            .after_inserted(clone!(input_ref => move |dom| {
                                *input_ref.borrow_mut() = Some(dom);
                            }))
                        })
                    },
                    Card::Image(data) => {
                        html!("empty-fragment", {
                            .child_signal(data.signal_cloned().map(clone!(state, editing_active => move |data| {
                                Some(match data {
                                    None => {
                                        html!("img-ui", {
                                            .property("path", "core/_common/image-empty.svg")
                                            .event_preventable(clone!(state, editing_active => move |evt:events::DragOver| {
                                                if let Some(data_transfer) = evt.data_transfer() {
                                                    if data_transfer.types().index_of(&JsValue::from_str(IMAGE_SEARCH_DATA_TRANSFER), 0) != -1 {
                                                        evt.prevent_default();
                                                        editing_active.set_neq(true);
                                                    }
                                                }

                                            }))
                                            .event(clone!(editing_active => move |evt:events::DragLeave| {
                                                editing_active.set_neq(false);
                                            }))
                                            .event(clone!(state, index, editing_active => move |evt:events::Drop| {
                                                if let Some(data_transfer) = evt.data_transfer() {
                                                    if let Some(data) = data_transfer.get_data(IMAGE_SEARCH_DATA_TRANSFER).ok() { 
                                                        let data:ImageDataTransfer = serde_json::from_str(&data).unwrap_ji();
                                                        let index = index.get().unwrap_or_default();
                                                        state.replace_card_image(index, side, (data.id, data.lib));
                                                    }
                                                }
                                                editing_active.set_neq(false);
                                            }))
                                        })
                                    },
                                    Some(data) => {
                                        html!("img-ji", {
                                            .property("size", "full")
                                            .property("id", data.0.0.to_string())
                                            .property("lib", data.1.to_str())
                                        })
                                    }
                                })
                            })))
                        })
                    },
                    _ => unimplemented!("can't render other types yet!")
                }
            })
        })
    }
}
