use std::rc::Rc;

use dominator::{clone, events, html, with_node, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::jig::report::JigReportType;
use strum::IntoEnumIterator;
use web_sys::HtmlSelectElement;

use super::super::state::{ReportStatus, Sidebar};

const STR_REPORT: &str = "Report";
const STR_REPORT_SENT: &str = "Your report has been sent.";

pub fn render(state: Rc<Sidebar>) -> impl Signal<Item = Vec<Dom>> {
    state
        .report_status
        .signal_cloned()
        .map(clone!(state => move|report_status| {
            match report_status {
                ReportStatus::Default => render_default(Rc::clone(&state)),
                ReportStatus::Active => render_active(Rc::clone(&state)),
                ReportStatus::Sent => render_sent(Rc::clone(&state)),
            }
        }))
}

fn render_button(state: Rc<Sidebar>) -> Dom {
    html!("button-rect", {
        .prop("slot", "report")
        .prop("color", "blue")
        .text(STR_REPORT)
        .event(clone!(state => move |_: events::Click| {
            state.report_status.set(ReportStatus::Active);
        }))
    })
}

fn render_default(state: Rc<Sidebar>) -> Vec<Dom> {
    vec![render_button(state)]
}

fn render_sent(state: Rc<Sidebar>) -> Vec<Dom> {
    vec![
        render_button(state),
        html!("span", {
            .prop("slot", "report-sent")
            .text(STR_REPORT_SENT)
        }),
    ]
}

fn render_active(state: Rc<Sidebar>) -> Vec<Dom> {
    vec![html!("jig-play-sidebar-report", {
        .prop("slot", "report")
        .children(&mut [
            html!("select" => HtmlSelectElement, {
                .with_node!(select => {
                    .prop("slot", "select")
                    .child(html!("option"))
                    .children(JigReportType::iter().map(|option| {
                        html!("option", {
                            .prop("value", option.to_value_str())
                            .text(option.as_str())
                        })
                    }))
                    .event(clone!(state => move |_: events::Change| {
                        let value = select.value();
                        if value.is_empty() {
                            state.report_type.set(None);
                        } else {
                            let report_type = JigReportType::from_value_str(&value);
                            state.report_type.set(Some(report_type));
                        }
                    }))
                })
            }),
            html!("button", {
                .prop("slot", "button")
                .text(STR_REPORT)
                .event(clone!(state => move |_: events::Click| {
                    state.send_report();
                }))
            }),
        ])
    })]
}
