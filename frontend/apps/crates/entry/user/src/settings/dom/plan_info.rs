use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::billing::{PaymentMethodType, PaymentNetwork, PlanType};
use std::rc::Rc;
use utils::{events, prelude::plan_type_signal};

use crate::settings::state::{PlanSectionInfo, SettingsPage};

const STR_RENEWS_ON: &str = "Renews on";
const STR_EXPIRES_ON: &str = "Expires on";

#[derive(Clone, Copy, Debug, strum_macros::EnumIs)]
enum PaymentFrequency {
    Annually,
    Monthly,
}
fn plan_payment_frequency(plan_type: PlanType) -> PaymentFrequency {
    match plan_type {
        PlanType::IndividualBasicMonthly | PlanType::IndividualProMonthly => {
            PaymentFrequency::Monthly
        }
        PlanType::IndividualBasicAnnually
        | PlanType::IndividualProAnnually
        | PlanType::SchoolLevel1
        | PlanType::SchoolLevel2
        | PlanType::SchoolLevel3
        | PlanType::SchoolLevel4
        | PlanType::SchoolUnlimited => PaymentFrequency::Annually,
    }
}

impl SettingsPage {
    pub(super) fn render_plan_section(self: &Rc<Self>, plan_info: &PlanSectionInfo) -> Vec<Dom> {
        let state = self;
        let auto_renew = plan_info.auto_renew.read_only();
        let price = plan_info.price;
        vec![
            html!("p", {
                .prop("slot", "plan-type")
                .text_signal(plan_type_signal().map(|plan| {
                    plan.map(|plan| {
                        plan.display_name()
                    }).unwrap_or_default()
                }))
            }),
            html!("p", {
                .prop("slot", "plan-price")
                .text_signal(plan_type_signal().map(move |plan_type| {
                    plan_type.map(|plan_type| {
                        let frequency = plan_payment_frequency(plan_type);
                        price_string(price, frequency)
                    }).unwrap_or_default()
                }))
            }),
            html!("p", {
                .prop("slot", "plan-renews-on")
                .text(&plan_info.current_period_end.format("%h %e, %Y").to_string())
            }),
            html!("p", {
                .prop("slot", "plan-renewal-label")
                .text_signal(auto_renew.signal().map(|auto_renew| match auto_renew {
                    true => STR_RENEWS_ON,
                    false => STR_EXPIRES_ON,
                }))
            }),
            html!("div", {
                .prop("slot", "plan-auto-renew")
                .child(html!("input-switch", {
                    .prop_signal("enabled", auto_renew.signal())
                    .event(clone!(state, auto_renew => move|_ :events::CustomToggle| {
                        state.set_auto_renew(!auto_renew.get());
                    }))
                }))
                .child(html!("span", {
                    .text_signal(auto_renew.signal().map(|auto_renew| match auto_renew {
                        true => "On",
                        false => "Off",
                    }))
                }))
            }),
            state.render_payment_method(&plan_info.payment_method_type),
            html!("div", {
                .prop("slot", "change-to-annual")
                .child_signal(plan_type_signal().map(clone!(state => move |plan_type| {
                    let frequency = plan_payment_frequency(plan_type?);
                    match frequency {
                        PaymentFrequency::Monthly => {
                            Some(html!("button-rect", {
                                .prop("slot", "change-to-annual")
                                .prop("type", "filled")
                                .prop("color", "blue")
                                .text("Get 2 months FREE by switching to yearly")
                                .event(clone!(state => move|_ :events::Click| {
                                    state.change_to_annual_billing();
                                }))
                            }))
                        },
                        PaymentFrequency::Annually => None,
                    }
                })))
            }),
        ]
    }

    fn render_payment_method(self: &Rc<Self>, payment_method_type: &PaymentMethodType) -> Dom {
        html!("div", {
            .prop("slot", "plan-payment-method")
            .child(html!("img-ui", {
                .style("height", "22px")
                .prop("path", payment_method_type_icon(&payment_method_type))
            }))
            .apply(|mut dom| {
                if let PaymentMethodType::Card(card) = &payment_method_type {
                    dom = dom.child(html!("span", {
                        .text("••••")
                        .text(&card.last4.to_string())
                    }));
                }
                dom
            })
        })
    }
}

fn payment_method_type_icon(method_type: &PaymentMethodType) -> &'static str {
    match method_type {
        PaymentMethodType::ApplePay => "payment-method/apple-pay.svg",
        PaymentMethodType::GooglePay => "payment-method/google-pay.svg",
        PaymentMethodType::Link => "payment-method/??.svg",
        PaymentMethodType::Card(card) => match card.payment_network {
            PaymentNetwork::Visa => "payment-method/visa.svg",
            PaymentNetwork::Mastercard => "payment-method/mastercard.svg",
            PaymentNetwork::Discover => "payment-method/discover.svg",
            PaymentNetwork::JCB => "payment-method/jcb.svg",
            PaymentNetwork::AmericanExpress => "payment-method/american-express.svg",
            PaymentNetwork::UnionPay => "payment-method/??.svg",
            PaymentNetwork::DinersClub => "payment-method/??.svg",
            PaymentNetwork::Unknown => "payment-method/??.svg",
        },
        PaymentMethodType::Other => "payment-method/??.svg",
    }
}
fn price_string(price: u32, frequency: PaymentFrequency) -> String {
    let frequency = match frequency {
        PaymentFrequency::Annually => "per year",
        PaymentFrequency::Monthly => "per month",
    };
    let price = price as f32 / 100.0;
    format!("${price:.2} - {frequency}")
}
