use super::state::*;
use crate::overlay::handle::OverlayHandle;
use crate::transform::actions::focus_within;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::prelude::*;
use utils::resize::resize_info_signal;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn render_transform(
    state: Rc<TransformState>,
    resize_level: ResizeLevel,
    get_menu_contents: Option<impl Fn() -> Dom + 'static>,
) -> Dom {
    let get_menu_contents = get_menu_contents.map(Rc::new);

    html!("empty-fragment", {
        .child(
            html!("transform-box", {
                .after_inserted(clone!(state => move |elem| {
                    *state.dom_ref.borrow_mut() = Some(elem.unchecked_into());
                }))
                .after_removed(clone!(state => move |_elem| {
                    *state.dom_ref.borrow_mut() = None;
                }))
                .child(html!("button-icon" => HtmlElement, {
                    .prop("slot", "menu-btn")
                    .prop("icon", "circle-kebab-grey")
                    .style("display", "block")
                    .style_signal("transform", state.invert_rotation_matrix_string_signal())
                    .with_node!(elem => {
                        .event(clone!(state => move |_evt:events::Click| {
                            let dom_rect = elem.get_bounding_client_rect();
                            let x = dom_rect.x();
                            let y = dom_rect.y();
                            state.menu_pos.set(Some((x, y)));
                        }))
                    })
                }))
                .style("display", "block")

                .style("position", "absolute")
                .style_signal("transform", state.rotation_matrix_string_signal())
                .style_signal("top", state.y_px_signal().map(|x| format!("{}px", x)))
                .style_signal("left", state.x_px_signal().map(|x| format!("{}px", x)))
                .style_signal("width", state.width_px_signal().map(|x| format!("{}px", x)))
                .style_signal("height", state.height_px_signal().map(|x| format!("{}px", x)))
                .prop_signal("isTransforming", state.is_transforming.signal())
                .prop("hasMenu", get_menu_contents.is_some())
                .prop("resizeLevel", resize_level.to_str())
                .prop_signal("width", state.width_px_signal())
                .prop_signal("height", state.height_px_signal())
                .prop_signal("screenScale", resize_info_signal().map(|resize| resize.scale))
                .event(clone!(state => move |_evt:super::events::RectDblClick| {
                    if let Some(on_double_click) = &state.callbacks.on_double_click {
                        (on_double_click) ();
                    }
                }))
                .event(clone!(state => move |evt:super::events::Move| {
                    let data = evt.data();
                    state.start_tracking_action(Action::Move, data.x as i32, data.y as i32);
                }))
                .event(clone!(state => move |evt:super::events::Rotate| {
                    let data = evt.data();
                    state.start_tracking_action(Action::Rotate, data.x as i32, data.y as i32);
                }))
                .event(clone!(state => move |evt:super::events::Resize| {
                    let data = evt.data();
                    let from = data.scale_from();


                    let lock_aspect = !*state.alt_pressed.borrow();
                    state.start_tracking_action(Action::Scale(from, lock_aspect), data.x as i32, data.y as i32);
                }))
                .global_event(clone!(state => move |evt:events::KeyDown| {
                    let mut transform = state.transform.lock_mut();
                    let key = Key::from(evt.key());

                    match key {
                        Key::Shift => *state.shift_pressed.borrow_mut() = true,
                        Key::Alt => *state.alt_pressed.borrow_mut() = true,
                        _ => {},
                    }

                    if key.is_move_key() {
                        let current = transform.get_translation_2d();
                        let mut translation = key.translation_from_key();

                        if *state.shift_pressed.borrow() {
                            translation.0 *= MOVE_MULTIPLIER;
                            translation.1 *= MOVE_MULTIPLIER;
                        }

                        let next = (current.0 + translation.0, current.1 + translation.1);
                        transform.set_translation_2d(next.0, next.1);
                    }
                }))
                .global_event(clone!(state => move |evt:events::KeyUp| {
                    let key = Key::from(evt.key());
                    match key {
                        Key::Shift => *state.shift_pressed.borrow_mut() = false,
                        Key::Alt => *state.alt_pressed.borrow_mut() = false,
                        _ => {},
                    }

                    if key.is_move_key() {
                        if let Some(on_action_finished) = &state.callbacks.on_action_finished {
                            on_action_finished(state.transform.get_cloned());
                        }
                    }
                }))

                .global_event(clone!(state => move |evt:events::PointerUp| {
                    state.stop_tracking_action(evt.x() as i32, evt.y() as i32);
                }))
                .global_event(clone!(state => move |evt:events::PointerMove| {
                    state.mouse_move(evt.x() as i32, evt.y() as i32);
                }))
                .with_node!(elem => {
                    .event(clone!(state, elem => move |_: events::FocusOut| {
                        state.on_focus_out(&elem);
                    }))
                })
                .after_inserted(|elem| {
                    wasm_bindgen_futures::spawn_local(clone!(elem => async move {
                        gloo_timers::future::TimeoutFuture::new(0).await;
                        // automatically focus so that blur works
                        let _ = elem.focus();
                    }));
                })
            })
        )
        .child_signal(
            state
                .menu_pos.signal_cloned()
                .map(clone!(state => move |pos| {
                    match get_menu_contents.as_ref() {
                        None => None,
                        Some(get_menu_contents) => {
                            pos.map(|pos| {
                                html!("empty-fragment", {
                                    .apply(OverlayHandle::lifecycle(
                                        clone!(pos, state, get_menu_contents => move || {
                                            html!("overlay-drag" => HtmlElement, {
                                                .with_node!(elem => {
                                                    .prop("target", web_sys::DomRect::new_with_x_and_y_and_width_and_height(pos.0 + 32.0, pos.1, 1.0, 1.0).unwrap_ji())
                                                    .child(html!("menu-container", {
                                                        .child((get_menu_contents.as_ref())())
                                                    }))
                                                    .event(clone!(state => move |_evt:events::Close| {
                                                        state.menu_pos.set(None);
                                                    }))
                                                    .event(clone!(state, elem => move |_: events::FocusOut| {
                                                        if !focus_within(&elem) {
                                                            if let Some(on_blur) = &state.callbacks.on_blur {
                                                                (on_blur) ();
                                                            }
                                                        };
                                                    }))
                                                    .after_inserted(clone!(state => move |elem| {
                                                        *state.overlay_drag_elem.borrow_mut() = Some(elem);
                                                    }))
                                                    .after_removed(clone!(state => move|_|{
                                                        *state.overlay_drag_elem.borrow_mut() = None;
                                                    }))
                                                })
                                            })
                                        })
                                    ))
                                })
                            })
                        }
                    }
                }))
        )
    })
}
