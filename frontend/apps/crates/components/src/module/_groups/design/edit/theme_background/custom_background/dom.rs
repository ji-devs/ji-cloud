use std::rc::Rc;

use crate::{
    color_select::dom::render as render_color_picker,
    image::search::{
        dom::render_with_action as render_image_search_with_action,
        state::State as ImageSearchState,
    },
    module::{
        _common::edit::entry::prelude::BaseExt, _groups::design::edit::design_ext::DesignExt,
    },
    overlay::handle::OverlayHandle,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::{ModeExt, StepExt};
use utils::events;
use web_sys::HtmlElement;

use super::super::state::STR_DESIGN_FROM_SCRATCH;
use super::state::{CustomBackground, Tab};

const STR_FILL_COLOR: &str = "Fill color";

impl<Step, Mode, Base> CustomBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("theme-custom-background", {
            .prop("header", STR_DESIGN_FROM_SCRATCH)
            .prop("tabbed", true)
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    (state.on_close)();
                }))
            }))
            .child(html!("menu-tabs", {
                .future(state.tab.signal_ref(|tab| tab.kind()).dedupe().for_each(clone!(state => move |kind| {
                    state.tab_kind.set(Some(kind));
                    async move {}
                })))
                .children(&mut [
                    state.render_tab(MenuTabKind::BackgroundImage),
                    state.render_tab(MenuTabKind::Overlay),
                    html!("module-sidebar-body", {
                        .prop("slot", "body")
                        .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                            let state = Rc::clone(&state);
                            match tab {
                                Tab::BackgroundImage(image_state) => {
                                    Some(state.render_background_tab_body(image_state))
                                },
                                Tab::Overlay(overlay_state) => {
                                    Some(state.render_overlay_tab_body(overlay_state))
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

    fn render_background_tab_body(self: &Rc<Self>, image_state: Rc<ImageSearchState>) -> Dom {
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

    fn render_overlay_tab_body(self: &Rc<Self>, overlay_state: Rc<ImageSearchState>) -> Dom {
        let state = self;
        let delete = clone!(state => move|| {
            html!("button-rect", {
                .prop("kind", "text")
                .prop("color", "blue")
                .child(html!("fa-icon", {
                    .prop("icon", "fa-light fa-trash-can")
                    .event(clone!(state => move |_: events::Click| {
                        state.remove_overlay();
                    }))
                }))
            })
        });
        render_image_search_with_action(overlay_state, None, Some(delete))
    }
}
