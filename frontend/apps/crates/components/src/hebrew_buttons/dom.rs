use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use utils::events;
use web_sys::HtmlElement;

use crate::overlay::handle::OverlayHandle;

use super::state::{HebrewButtons, Popup};

const SEFARIA_URL: &str = "https://www.sefaria.org/texts/Tanakh";
const SEFARIA_IFRAME_WIDTH: u32 = 700;
const DICTA_URL: &str = "https://embed.dicta.org.il/";
const DICTA_IFRAME_WIDTH: u32 = 550;

impl HebrewButtons {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        html!("hebrew-buttons" => HtmlElement, {
            .with_node!(elem => {
                .property_signal("full", state.active_popup.signal().map(clone!(state => move |active_popup| {
                    state.full || active_popup.is_some()
                })))
                .apply_if(slot.is_some(), |dom| {
                    dom.property("slot", slot.unwrap())
                })
                .child_signal(state.active_popup.signal().map(clone!(state, elem => move|active_popup| {
                    active_popup.map(|popup| state.render_popups(popup, elem.clone()))
                })))
                .children(&mut [
                    html!("hebrew-inputs-action", {
                        .property("slot", "full-only")
                        .property("kind", Popup::Sefaria.str())
                        .property_signal("active", state.active_popup.signal().map(|active_popup| {
                            active_popup == Some(Popup::Sefaria)
                        }))
                        .event(clone!(state => move|_: events::Click| {
                            state.on_action_click(Popup::Sefaria);
                        }))
                    }),
                    html!("div", {
                        .property("slot", "full-only")
                        .class("divider")
                    }),
                    html!("hebrew-inputs-action", {
                        .property("slot", "full-only")
                        .property("kind", Popup::Dicta.str())
                        .property_signal("active", state.active_popup.signal().map(|active_popup| {
                            active_popup == Some(Popup::Dicta)
                        }))
                        .event(clone!(state => move|_: events::Click| {
                            state.on_action_click(Popup::Dicta);
                        }))
                    }),
                    html!("div", {
                        .property("slot", "full-only")
                        .class("divider")
                    }),
                    html!("hebrew-inputs-action", {
                        .property("slot", "always")
                        .property("kind", Popup::Keyboard.str())
                        .property_signal("active", state.active_popup.signal().map(|active_popup| {
                            active_popup == Some(Popup::Keyboard)
                        }))
                        .event(clone!(state => move|_: events::Click| {
                            state.on_action_click(Popup::Keyboard);
                        }))
                    }),
                ])
            })
        })
    }

    fn render_popups(self: &Rc<Self>, active_popup: Popup, elem: HtmlElement) -> Dom {
        let state = Rc::clone(self);
        html!("empty-fragment" => HtmlElement, {
            .apply(OverlayHandle::lifecycle(
                clone!(elem => move || {
                    match active_popup {
                        Popup::Keyboard => {
                            html!("overlay-drag", {
                                .property("target", &elem)
                                .property("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-keyboard")))
                            })
                        },
                        Popup::Dicta => {
                            html!("overlay-content", {
                                .property("target", &elem)
                                .property("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-inputs-iframe", {
                                    .property("src", DICTA_URL)
                                    .property("width", DICTA_IFRAME_WIDTH)
                                })))
                            })
                        },
                        Popup::Sefaria => {
                            html!("overlay-content", {
                                .property("target", &elem)
                                .property("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-inputs-iframe", {
                                    .property("src", SEFARIA_URL)
                                    .property("width", SEFARIA_IFRAME_WIDTH)
                                })))
                            })
                        },
                    }
                })
            ))
        })
    }

    fn render_closable_popup(self: &Rc<Self>, dom: Dom) -> Dom {
        let state = Rc::clone(self);

        html!("hebrew-inputs-closable-popup", {
            .event(clone!(state => move|_: events::Close| {
                state.active_popup.set(None);
            }))
            .child(dom)
        })
    }
}
