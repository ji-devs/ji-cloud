use components::stickers::embed::types::{ParseUrlExt, PartialGoogleSheetsEmbed};
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::_groups::design::GoogleSheetId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use crate::base::sidebar::step_2::actions;

use super::super::state::Step2;

pub fn render_google_sheet(state: &Rc<Step2>, google_sheet: &Rc<PartialGoogleSheetsEmbed>) -> Dom {
    html!("div", {
        .child(html!("input-wrapper" => HtmlElement, {
            .with_node!(wrapper => {
                .prop("slot", "input")
                .prop("label", "Add a Google Sheet link")
                .child(html!("input" => HtmlInputElement, {
                    .prop_signal("value", google_sheet.url.signal_cloned().map(|url| {
                        match url {
                            Some(url) => url.0.clone(),
                            None => String::new(),
                        }
                    }))
                    .with_node!(input => {
                        .event(clone!(state, google_sheet => move |_: events::Input| {
                            match GoogleSheetId::try_parse(input.value()) {
                                Err(_) => {
                                    actions::set_error(&wrapper, true);
                                    google_sheet.url.set(None);
                                }
                                Ok(google_sheet_url) => {
                                    actions::set_error(&wrapper, false);
                                    google_sheet.url.set(Some(google_sheet_url));
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
