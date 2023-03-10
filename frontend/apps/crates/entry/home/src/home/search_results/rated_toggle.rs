use std::rc::Rc;

use crate::home::Home;
use dominator::{clone, html};
use futures_signals::signal::{Mutable, SignalExt};
use utils::{component::Component, events};

use super::super::actions::search;

pub struct RatedToggleConfig {
    pub slot: Option<&'static str>,
    pub is_rated: Mutable<bool>,
    pub home_state: Rc<Home>,
}

pub struct RatedToggle {
    config: RatedToggleConfig,
}

impl RatedToggle {
    pub fn new(config: RatedToggleConfig) -> Rc<Self> {
        Rc::new(Self { config })
    }
}

impl Component<RatedToggle> for Rc<RatedToggle> {
    fn styles() -> &'static str {
        r#"
            :host {
                background-color: var(--light-gray-1);
                display: grid;
                grid-template-columns: 1fr 1fr;
                border-radius: 10px;
                height: 40px;
            }
            button {
                all: unset;
                cursor: pointer;
                grid-row: 1;
                display: grid;
                place-content: center;
                z-index: 1;
            }
            button.rated {
                grid-column: 1;
            }
            button.all {
                grid-column: 2;
            }
            .indicator-wrapper {
                grid-row: 1;
                grid-column: 1 / -1;
                padding: 5px;
                display: grid;
                grid-template-columns: 1fr 1fr;
                /* column-gap: 16px; */
            }
            .indicator {
                background-color: #ffffff;
                translate: 0;
                transition: translate .2s;
                border-radius: 6px;
            }
            .indicator.all {
                translate: 100%;
            }
        "#
    }

    fn apply_on_host(
        &self,
        host: dominator::DomBuilder<web_sys::HtmlElement>,
    ) -> dominator::DomBuilder<web_sys::HtmlElement> {
        match self.config.slot {
            Some(slot) => host.prop("slot", slot),
            None => host,
        }
    }

    fn dom(
        &self,
        dom: dominator::DomBuilder<web_sys::ShadowRoot>,
    ) -> dominator::DomBuilder<web_sys::ShadowRoot> {
        let state = self;
        dom.child(html!("div", {
            .class("indicator-wrapper")
            .child(html!("div", {
                .class("indicator")
                .class_signal("all", self.config.is_rated.signal().map(|x| !x))
            }))
        }))
        .child(html!("button", {
            .class("rated")
            .text("Top rated")
            .event(clone!(state => move |_: events::Click| {
                state.config.is_rated.set(true);
                search(&state.config.home_state);
            }))
        }))
        .child(html!("button", {
            .class("all")
            .text("All")
            .event(clone!(state => move |_: events::Click| {
                state.config.is_rated.set(false);
                search(&state.config.home_state);
            }))
        }))
    }
}
