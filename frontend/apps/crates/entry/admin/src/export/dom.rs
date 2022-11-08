use super::Export;
use dominator::{clone, html, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use serde::Serialize;
use shared::api::PathParts;
use shared::{
    api::{endpoints::admin::ExportData, ApiEndpoint},
    domain::admin::{DateFilterType, ExportType},
};
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::{events, init::settings::SETTINGS, unwrap::UnwrapJiExt};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;

impl Export {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        html!("admin-export", {
            .child(html!("input-select", {
                .prop("label", "Data to export")
                .prop("multiple", false)
                .prop_signal("value", state.export_type.signal_cloned().map(|export_type| {
                        JsValue::from_str(&format!("{}", export_type))
                }))
                .children(ExportType::iter().map(|export_type| {
                    html!("input-select-option", {
                        .text(&format!("{}", export_type))
                        .prop_signal("selected", state.export_type.signal_cloned().map(clone!(export_type => move |current_type| {
                            export_type == current_type
                        })))
                        .event(clone!(state => move |_: events::CustomSelectedChange| {
                            state.export_type.set(export_type.clone());
                        }))
                    })
                }))
            }))
            .child(html!("input-select", {
                .prop("label", "Filter by")
                .prop("multiple", false)
                .prop_signal("value", state.date_filter_type.signal_cloned().map(|date_filter_type| {
                        JsValue::from_str(&format!("{}", date_filter_type))
                }))
                .children(DateFilterType::iter().map(|date_filter_type| {
                    html!("input-select-option", {
                        .text(&format!("{}", date_filter_type))
                        .prop_signal("selected", state.date_filter_type.signal_cloned().map(clone!(date_filter_type => move |current_type| {
                            date_filter_type == current_type
                        })))
                        .event(clone!(state => move |_: events::CustomSelectedChange| {
                            state.date_filter_type.set(date_filter_type.clone());
                        }))
                    })
                }))
            }))
            .child(html!("input-wrapper", {
                .prop("label", "From date")
                .child(html!("input" => HtmlInputElement, {
                    .prop("type", "date")
                    .prop("pattern", "\\d{4}-\\d{2}-\\d{2}")
                    .with_node!(input => {
                        .event(clone!(state => move |_: events::Input| {
                            state.set_date(&state.from_date, &input.value().into());
                        }))
                    })
                }))
            }))
            .child(html!("input-wrapper", {
                .prop("label", "To date")
                .child(html!("input" => HtmlInputElement, {
                    .prop("type", "date")
                    .prop("pattern", "\\d{4}-\\d{2}-\\d{2}")
                    .with_node!(input => {
                        .event(clone!(state => move |_: events::Input| {
                            state.set_date(&state.to_date, &input.value().into());
                        }))
                    })
                }))
            }))
            .child(html!("button-rect", {
                .prop("kind", "filled")
                .prop("color", "blue")
                .prop_signal("href", map_ref! {
                    let export_type = state.export_type.signal_cloned(),
                    let date_filter_type = state.date_filter_type.signal_cloned(),
                    let from_date = state.from_date.signal_cloned(),
                    let to_date = state.to_date.signal_cloned()
                        => {
                            let mut params = Vec::new();

                            params.push(format!("export_type={}", serializable_as_str(export_type)));
                            params.push(format!("date_filter_type={}", serializable_as_str(date_filter_type)));

                            if let Some(from_date) = from_date {
                                params.push(format!("from_date={}", from_date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)));
                            }
                            if let Some(to_date) = to_date {
                                params.push(format!("to_date={}", to_date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)));
                            }
                            let remote = SETTINGS.get().unwrap_ji().remote_target.api_url();
                            format!("{}{}?{}", remote, <ExportData as ApiEndpoint>::Path::PATH, params.join("&"))
                        }
                })
                .text("Export")
            }))
        })
    }
}

fn serializable_as_str<T: Serialize>(value: T) -> String {
    let value = serde_json::to_value(value).unwrap_ji();
    value.as_str().unwrap_ji().to_owned()
}
