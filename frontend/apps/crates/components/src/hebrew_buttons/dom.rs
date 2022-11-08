use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlElement;

use crate::overlay::handle::OverlayHandle;

use super::{
    state::{HebrewButtons, Popup},
    HebrewButtonOpened, Kind,
};

const SEFARIA_URL: &str = "https://www.sefaria.org/texts/Tanakh";
const SEFARIA_IFRAME_WIDTH: u32 = 700;
const DICTA_URL: &str = "https://embed.dicta.org.il/";
const DICTA_IFRAME_WIDTH: u32 = 550;

impl HebrewButtons {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;
        html!("hebrew-buttons" => HtmlElement, {
            .with_node!(elem => {
                .future(state.active_popup.signal().for_each(clone!(state => move |active_popup| {
                    (state.on_open_toggle)(active_popup.is_some());
                    async {}
                })))
                .prop_signal("full", state.active_popup.signal().map(clone!(state => move |active_popup| {
                    match state.kind {
                        Kind::Full => true,
                        Kind::Reveal => active_popup.is_some(),
                        Kind::KeyboardOnly => false,
                    }
                })))
                .apply_if(slot.is_some(), |dom| {
                    dom.prop("slot", slot.unwrap_ji())
                })
                .child_signal(state.active_popup.signal().map(clone!(state, elem => move|active_popup| {
                    active_popup.map(|popup| state.render_popups(popup, elem.clone()))
                })))
                .apply_if(state.kind != Kind::KeyboardOnly, |dom| {
                    dom.children(&mut [
                        html!("hebrew-inputs-action", {
                            .prop("slot", "full-only")
                            .prop("kind", Popup::Sefaria.str())
                            .prop_signal("active", state.active_popup.signal().map(|active_popup| {
                                active_popup == Some(Popup::Sefaria)
                            }))
                            .event(clone!(state => move|_: events::Click| {
                                state.on_action_click(Popup::Sefaria);
                            }))
                        }),
                        html!("div", {
                            .prop("slot", "full-only")
                            .class("divider")
                        }),
                        html!("hebrew-inputs-action", {
                            .prop("slot", "full-only")
                            .prop("kind", Popup::Dicta.str())
                            .prop_signal("active", state.active_popup.signal().map(|active_popup| {
                                active_popup == Some(Popup::Dicta)
                            }))
                            .event(clone!(state => move|_: events::Click| {
                                state.on_action_click(Popup::Dicta);
                            }))
                        }),
                        html!("div", {
                            .prop("slot", "full-only")
                            .class("divider")
                        }),
                    ])
                })
                .child(html!("hebrew-inputs-action", {
                    .prop("slot", "always")
                    .prop("kind", Popup::Keyboard.str())
                    .prop_signal("active", state.active_popup.signal().map(|active_popup| {
                        active_popup == Some(Popup::Keyboard)
                    }))
                    .event(clone!(state => move|_: events::Click| {
                        state.on_action_click(Popup::Keyboard);
                    }))
                }))
                .global_event(clone!(state => move|_: HebrewButtonOpened| {
                    state.active_popup.set_neq(None);
                }))
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
                                .prop("target", &elem)
                                .prop("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-keyboard")))
                            })
                        },
                        Popup::Dicta => {
                            html!("overlay-content", {
                                .prop("target", &elem)
                                .prop("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-inputs-iframe", {
                                    .prop("src", DICTA_URL)
                                    .prop("width", DICTA_IFRAME_WIDTH)
                                })))
                            })
                        },
                        Popup::Sefaria => {
                            html!("overlay-content", {
                                .prop("target", &elem)
                                .prop("contentAnchor", "rt")
                                .child(state.render_closable_popup(html!("hebrew-inputs-iframe", {
                                    .prop("src", SEFARIA_URL)
                                    .prop("width", SEFARIA_IFRAME_WIDTH)
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
