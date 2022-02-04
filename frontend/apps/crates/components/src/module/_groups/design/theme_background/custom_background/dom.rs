use std::rc::Rc;

use dominator::{html, Dom, clone, with_node};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::StepExt;
use utils::events;
use web_sys::HtmlElement;
use crate::{
    image::search::dom::render as render_image_search,
    image::search::dom::render_with_action as render_image_search_with_action,
    tabs::{MenuTab, MenuTabKind},
    overlay::handle::OverlayHandle,
    color_select::dom::render as render_color_picker,
    module::{_common::edit::entry::prelude::BaseExt, _groups::design::design_ext::DesignExt},
};

use super::state::{CustomBackground, Tab};

const STR_FILL_COLOR: &str = "Fill color";

impl<Step, Base> CustomBackground<Step, Base> where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("theme-custom-background", {
            .child(html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.on_close)();
                }))
            }))
            .child(html!("menu-tabs", {
                .future(state.tab.signal_ref(|tab| tab.as_index()).dedupe().for_each(clone!(state => move |index| {
                    state.tab_index.set(Some(index));
                    async move {}
                })))
                .children(&mut [
                    state.render_tab(MenuTabKind::BackgroundImage),
                    state.render_tab(MenuTabKind::Overlay),
                    html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                            let upper_state = Rc::clone(&state);
                            match tab {
                                Tab::BackgroundImage(state) => {
                                    let color = clone!(upper_state => move || {
                                        let state = Rc::clone(&upper_state);
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
                                                    clone!(elem => move || {
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
                                    Some(render_image_search_with_action(state, None, Some(color)))
                                },
                                Tab::Overlay(state) => {
                                    Some(render_image_search(state, None))
                                },
                            }
                        })))

                    })
                ])
            }))
        })
    }

    fn render_tab(self: &Rc<Self>, tab_kind: MenuTabKind) -> Dom {
        let state = self;
        MenuTab::render(
            MenuTab::new(
                tab_kind,
                false,
                true,
                clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                    curr.kind() == tab_kind
                }))),
                clone!(state, tab_kind => move || {
                    state.tab.set(Tab::new(state.base.clone(), tab_kind));
                }),
            ),
            Some("tabs"),
        )
    }
}
