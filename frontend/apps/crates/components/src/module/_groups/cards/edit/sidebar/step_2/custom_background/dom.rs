use std::rc::Rc;

use crate::{
    color_select::dom::render as render_color_picker,
    image::search::{
        dom::render_with_action as render_image_search_with_action,
        state::State as ImageSearchState,
    },
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
        html!("theme-custom-background", {
            .property("header", STR_CHANGE_BACKGROUND)
            .property("tabbed", false)
            .child(html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.on_close)();
                }))
            }))
            .child(state.render_background_tab_body(Rc::clone(&state.background_state)))
        })
    }

    fn render_background_tab_body(self: &Rc<Self>, image_state: Rc<ImageSearchState>) -> Dom {
        let state = self;
        let color = clone!(state => move || {
            html!("empty-fragment" => HtmlElement, {
                .with_node!(elem => {
                    .child(html!("button-rect", {
                        .property("kind", "text")
                        .property("color", "blue")
                        .child(html!("fa-icon", {
                            .property("icon", "fa-light fa-fill-drip")
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
                                .property("target", &elem)
                                .property("contentAnchor", "rt")
                                .property("marginX", 10)
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
                                                        .property("slot", "close")
                                                        .property("icon", "fa-light fa-xmark")
                                                        .event(clone!(state => move |_: events::Click| {
                                                            state.colors_open.set(false)
                                                        }))
                                                    }),
                                                    render_color_picker(
                                                        Rc::clone(&state.color_state),
                                                        Some("main")
                                                    )
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
        render_image_search_with_action(image_state, None, Some(color))
    }
}
