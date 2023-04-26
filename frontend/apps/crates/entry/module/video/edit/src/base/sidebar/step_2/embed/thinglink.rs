use components::stickers::embed::types::{ParseUrlExt, PartialThinglinkEmbed};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::ThinglinkId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

pub fn render_thinglink(state: &Rc<Step2>, thinglink: &Rc<PartialThinglinkEmbed>) -> Dom {
    html!("div", {
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .prop("slot", "input")
                .prop("label", "Add a Thinglink link")
                .child(html!("input" => HtmlInputElement, {
                    .prop_signal("value", thinglink.url.signal_cloned().map(|url| {
                        match url {
                            Some(url) => url.0.clone(),
                            None => String::new(),
                        }
                    }))
                    .with_node!(input => {
                        .event(clone!(state, thinglink => move |_: events::Input| {
                            match ThinglinkId::try_parse(input.value()) {
                                Err(_) => {
                                    actions::set_error(&wrapper, true);
                                    thinglink.url.set(None);
                                }
                                Ok(thinglink_url) => {
                                    actions::set_error(&wrapper, false);
                                    thinglink.url.set(Some(thinglink_url));
                                },
                            };
                            state.on_embed_value_change();
                        }))
                    })
                }))
            })
        }))
    })
}
