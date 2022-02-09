use chrono::TimeZone;
use dominator::{clone, html, Dom, with_node, text_signal};
use futures_signals::{signal::SignalExt, map_ref};
use shared::{
    api::{ApiEndpoint, endpoints::admin::ExportData},
    domain::admin::ExportType
};
use strum::IntoEnumIterator;
use utils::{init::settings::SETTINGS, events, unwrap::UnwrapJiExt};
use std::{rc::Rc, convert::AsRef};
use web_sys::{HtmlInputElement, HtmlElement};
use wasm_bindgen::prelude::*;
use super::Export;

impl Export {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("admin-export", {
            .child(html!("input-select", {
                .property("label", "Data to export")
                .property("multiple", false)
                .property_signal("value", state.export_type.signal_cloned().map(|export_type| {
                    match export_type {
                        None => JsValue::NULL,
                        Some(export_type) => JsValue::from_str(&format!("{}", export_type)),
                    }
                }))
                .children(ExportType::iter().map(|export_type| {
                    html!("input-select-option", {
                        .text(&format!("{}", export_type))
                        .property_signal("selected", state.export_type.signal_cloned().map(clone!(export_type => move |current_type| {
                            match current_type {
                                Some(current_type) => export_type == current_type,
                                None => false,
                            }
                        })))
                        .event(clone!(state => move |_: events::CustomSelectedChange| {
                            state.export_type.set(Some(export_type.clone()));
                        }))
                    })
                }))
            }))
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .property("label", "From date")
                    .child(html!("input" => HtmlInputElement, {
                        .property("type", "date")
                        .property("pattern", "\\d{4}-\\d{2}-\\d{2}")
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                state.set_date(&state.from_date, &input.value().into());
                            }))
                        })
                    }))
                })
            }))
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .property("label", "To date")
                    .child(html!("input" => HtmlInputElement, {
                        .property("type", "date")
                        .property("pattern", "\\d{4}-\\d{2}-\\d{2}")
                        .with_node!(input => {
                            .event(clone!(state => move |_: events::Input| {
                                state.set_date(&state.to_date, &input.value().into());
                            }))
                        })
                    }))
                })
            }))
            .child(html!("button-rect", {
                .property("kind", "filled")
                .property("color", "blue")
                .property_signal("disabled", state.export_type.signal_cloned().map(|export_type| export_type.is_none()))
                .property_signal("href", map_ref! {
                    let export_type = state.export_type.signal_cloned(),
                    let from_date = state.from_date.signal_cloned(),
                    let to_date = state.to_date.signal_cloned()
                        => {
                            let mut params = Vec::new();
                            if let Some(export_type) = export_type {
                                params.push(format!("export_type={}", export_type).to_lowercase());
                            }
                            if let Some(from_date) = from_date {
                                params.push(format!("from_date={}", from_date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)));
                            }
                            if let Some(to_date) = to_date {
                                params.push(format!("to_date={}", to_date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)));
                            }
                            let remote = SETTINGS.get().unwrap_ji().remote_target.api_url();
                            format!("{}{}?{}", remote, ExportData::PATH, params.join("&"))
                        }
                })
                .text("Export")
            }))
        })
    }
}
