use dominator::{Dom, html, clone, with_node};
use futures_signals::{
    signal::always,
    signal_vec::SignalVecExt
};
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::{Signal, SignalExt};
use crate::module::history::state::HistoryState;
use web_sys::HtmlElement;
use super::state::*;
use utils::resize::resize_info_signal;

pub struct TransformDom {}
//TODO - move on_undoredo into HistoryState itself
impl TransformDom {
    pub fn render(state: Rc<TransformState>, get_menu_contents: impl Fn() -> Dom + 'static) -> Dom {
        Self::render_child(state, get_menu_contents, || always(true), always(None))
    }

    pub fn render_child<M, F, A>(
        state: Rc<TransformState>, 
        get_menu_contents: M, 
        get_active_signal: F, 
        child_signal: impl Signal<Item = Option<Dom>> + 'static
    ) -> Dom 
    where
        M: Fn() -> Dom + 'static,
        F: Fn() -> A,
        A: Signal<Item = bool> + 'static, 
    {

        html!("empty-fragment", {
            .child(
                html!("transform-box", {
                    .child(html!("button-icon" => HtmlElement, {
                        .property("slot", "menu-btn")
                        .property("icon", "circle-kebab-grey")
                        .style("display", "block")
                        .style_signal("transform", state.invert_rotation_matrix_string_signal())
                        .with_node!(elem => {
                            .event(clone!(state => move |evt:events::Click| {
                                let dom_rect = elem.get_bounding_client_rect();
                                let x = dom_rect.x();
                                let y = dom_rect.y();
                                state.menu_pos.set(Some((x, y)));
                            }))
                        })
                    }))
                    .child_signal(child_signal)
                    .style("display", "block")

                    .style("position", "absolute")
                    .style_signal("transform", state.rotation_matrix_string_signal())
                    .style_signal("top", state.y_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("left", state.x_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("width", state.width_px_signal().map(|x| format!("{}px", x)))
                    .style_signal("height", state.height_px_signal().map(|x| format!("{}px", x)))
                    .property_signal("active", get_active_signal())
                    .property_signal("menuButtonVisible", state.menu_button_visible.signal())
                    .property_signal("width", state.width_px_signal())
                    .property_signal("height", state.height_px_signal())
                    .property_signal("rectHidden", state.rect_hidden.signal())
                    .property_signal("screenScale", resize_info_signal().map(|resize| resize.scale)) 
                    .event(clone!(state => move |evt:super::events::RectDblClick| {
                        if *state.hide_on_dbl_click.borrow() {
                            state.rect_hidden.set_neq(true);
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


                        let lock_aspect = !state.alt_pressed.borrow().clone();
                        state.start_tracking_action(Action::Scale(from, lock_aspect), data.x as i32, data.y as i32);
                    }))
                    .global_event(clone!(state => move |evt:events::KeyDown| {
                        if evt.key() == "Alt" {
                            *state.alt_pressed.borrow_mut() = true;
                        } 
                    }))
                    .global_event(clone!(state => move |evt:events::KeyUp| {
                        if evt.key() == "Alt" {
                            *state.alt_pressed.borrow_mut() = false;
                        } 
                    }))

                    .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
                        state.stop_tracking_action(evt.x() as i32, evt.y() as i32);
                    }))
                    .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
                        state.mouse_move(evt.x() as i32, evt.y() as i32);
                    }))

                })
            )
            .child_signal(
                state
                    .menu_pos_signal(get_active_signal())
                    .map(clone!(state => move |pos| {
                        pos.map(|pos| {
                            html!("drag-container", {
                                .style("position", "fixed")
                                .style("top", "0")
                                .style("left", "0")
                                .property("x", pos.0 + 32.0)
                                .property("y", pos.1)
                                .child(html!("menu-container", {
                                    .child(get_menu_contents())
                                }))
                                .event(clone!(state => move |evt:events::Close| {
                                    log::info!("GOT CLOSE!");
                                    state.menu_pos.set(None);
                                }))
                            })
                        })
                    }))
            )
        })
    }

}
