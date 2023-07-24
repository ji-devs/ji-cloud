use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::billing::{PaymentMethodType, PaymentNetwork, PlanType};
use std::rc::Rc;
use utils::{
    events,
    prelude::{get_plan_type, get_user_mutable},
    unwrap::UnwrapJiExt,
};

use crate::settings::state::{PlanSectionInfo, SettingsPage};

#[derive(Clone, Copy, strum_macros::EnumIs)]
enum PaymentFrequency {
    Annually,
    Monthly,
}
fn plan_payment_frequency() -> Option<PaymentFrequency> {
    let user = get_user_mutable();
    let user = user.lock_ref();
    let plan_type = user.as_ref()?.account_summary.as_ref()?.plan_type?;
    Some(match plan_type {
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
    })
}

impl SettingsPage {
    pub(super) fn render_plan_section(self: &Rc<Self>, plan_info: &PlanSectionInfo) -> Vec<Dom> {
        let state = self;
        let plan_type = get_plan_type().unwrap_ji();
        let payment_frequency = plan_payment_frequency().unwrap_ji();
        let auto_renew = plan_info.auto_renew.read_only();
        let mut output = vec![
            html!("p", {
                .prop("slot", "plan-type")
                .text(plan_type.display_name())
            }),
            html!("p", {
                .prop("slot", "plan-price")
                .text(&price_string(plan_info.price, payment_frequency))
            }),
            html!("p", {
                .prop("slot", "plan-renews-on")
                .text(&plan_info.current_period_end.format("%h %e, %Y").to_string())
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
        ];
        if payment_frequency.is_monthly() {
            output.push(html!("button-rect", {
                .prop("slot", "change-to-annual")
                .prop("type", "filled")
                .prop("color", "blue")
                .text("Get 2 months FREE by switching to yearly")
                .event(clone!(state => move|_ :events::Click| {
                    state.change_to_annual_billing();
                }))
            }))
        }
        output
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
        PaymentFrequency::Annually => "per month",
        PaymentFrequency::Monthly => "per year",
    };
    let price = price as f32 / 100.0;
    format!("${price:.2} - {frequency}")
}
