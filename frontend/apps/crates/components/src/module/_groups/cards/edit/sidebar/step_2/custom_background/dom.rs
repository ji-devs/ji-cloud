use std::rc::Rc;

use crate::{
    image::search::ImageSearch,
    module::_groups::cards::edit::state::{ExtraExt, RawDataExt},
    overlay::handle::OverlayHandle,
};
use dominator::{clone, html, with_node, Dom};
use utils::events;
use web_sys::HtmlElement;

use super::super::state::STR_CHANGE_BACKGROUND;
use super::state::CustomBackground;

const STR_FILL_COLOR: &str = "Fill color";

impl<RawData: RawDataExt, E: ExtraExt> CustomBackground<RawData, E> {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("module-sidebar-advanced-modal", {
            .prop("header", STR_CHANGE_BACKGROUND)
            .prop("tabbed", false)
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.on_close)();
                }))
            }))
            .child(state.render_background_tab_body(Rc::clone(&state.background_state)))
        })
    }

    fn render_background_tab_body(self: &Rc<Self>, image_state: Rc<ImageSearch>) -> Dom {
        let state = self;
        let color = clone!(state => move || {
            html!("empty-fragment" => HtmlElement, {
                .with_node!(elem => {
                    .child(html!("button-rect", {
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-light fa-fill-drip")
                        }))
                        .text(STR_FILL_COLOR)
                        .event(clone!(state => move |_: events::Click|{
                            let mut colors_open = state.colors_open.lock_mut();
                            *colors_open = !*colors_open;
                        }))
                    }))
                    .apply(OverlayHandle::lifecycle(
                        clone!(state, elem => move || {
                            html!("overlay-content", {
                                .prop("target", &elem)
                                .prop("contentAnchor", "rt")
                                .prop("marginX", 10)
                                .child_signal(state.colors_open.signal_ref(clone!(state => move |colors_open| {
                                    match colors_open {
                                        false => None,
                                        true => {
                                            Some(html!("theme-background-color", {
                                                .event(clone!(state => move |_: events::Close| {
                                                    state.colors_open.set(false);
                                                }))
                                                .children(&mut [
                                                    html!("fa-button", {
                                                        .prop("slot", "close")
                                                        .prop("icon", "fa-light fa-xmark")
                                                        .event(clone!(state => move |_: events::Click| {
                                                            state.colors_open.set(false)
                                                        }))
                                                    }),
                                                    state.color_state.render(Some("main"))
                                                ])
                                            }))
                                        },
                                    }
                                })))
                            })
                        })
                    ))
                })
            })
        });
        image_state.render_with_action(None, Some(color))
    }
}
