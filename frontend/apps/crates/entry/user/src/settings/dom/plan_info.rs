use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::billing::{
    AmountInCents, AppliedCoupon, PaymentMethodType, PaymentNetwork, PlanType,
};
use std::rc::Rc;
use utils::{events, prelude::plan_type_signal};

use crate::settings::state::{PlanSectionInfo, SettingsPage};

const STR_EXPIRED_ON: &str = "Expired on";
const STR_TRIAL_ENDS_ON: &str = "Trial ends on";
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
    pub(super) fn render_plan_section(
        self: &Rc<Self>,
        plan_info: &Rc<PlanSectionInfo>,
    ) -> Vec<Dom> {
        let state = self;
        let auto_renew = plan_info.auto_renew.read_only();

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
                .text_signal(plan_type_signal().map(clone!(plan_info => move |plan_type| {
                    plan_type.map(|plan_type| {
                        let frequency = plan_payment_frequency(plan_type);
                        price_string(&plan_info.price, &plan_info.coupon, frequency)
                    }).unwrap_or_default()
                })))
            }),
            html!("p", {
                .prop("slot", "plan-renews-on")
                .text(&plan_info.current_period_end.format("%h %e, %Y").to_string())
            }),
            html!("p", {
                .prop("slot", "plan-renewal-label")
                .text_signal(auto_renew.signal().map(clone!(plan_info => move |auto_renew| {
                    if plan_info.is_trial {
                        STR_TRIAL_ENDS_ON
                    } else if !plan_info.status.is_valid() {
                        STR_EXPIRED_ON
                    } else {
                        match auto_renew {
                            true => STR_RENEWS_ON,
                            false => STR_EXPIRES_ON,
                        }
                    }
                })))
            }),
            html!("div", {
                .prop("slot", "plan-auto-renew")
                .child(html!("input-switch", {
                    .prop("disabled", !plan_info.status.is_valid())
                    .prop_signal("enabled", auto_renew.signal().map(clone!(plan_info => move |auto_renew| {
                        if !plan_info.status.is_valid() {
                            false
                        } else {
                            auto_renew
                        }
                    })))
                    .event(clone!(state, plan_info, auto_renew => move|_ :events::CustomToggle| {
                        if plan_info.status.is_valid() {
                            state.set_auto_renew(!auto_renew.get());
                        }
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

    fn render_payment_method(
        self: &Rc<Self>,
        payment_method_type: &Option<PaymentMethodType>,
    ) -> Dom {
        html!("div", {
            .prop("slot", "plan-payment-method")
            .apply(|dom| {
                match payment_method_type {
                    None => {
                        dom.child(html!("span", {
                            .text("-")
                        }))
                    },
                    Some(payment_method_type) => {
                        dom.child(html!("img-ui", {
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
                    },
                }
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
fn price_string(
    price: &AmountInCents,
    coupon: &Option<AppliedCoupon>,
    frequency: PaymentFrequency,
) -> String {
    let frequency = match frequency {
        PaymentFrequency::Annually => "pa",
        PaymentFrequency::Monthly => "pm",
    };

    let discounted = AmountInCents::from(
        coupon
            .as_ref()
            .and_then(|coupon| coupon.coupon_percent)
            .map_or(price.inner() as f64, |percent| {
                let price = price.inner() as f64;
                price - (price * f64::from(percent))
            }) as i64,
    );

    let coupon = coupon
        .as_ref()
        .map(|coupon| {
            let applied_to = coupon
                .coupon_to
                .map(|to| format!(" until {}", to.date_naive().to_string()))
                .unwrap_or_default();
            format!(
                " ({} off with {}{applied_to})",
                coupon.coupon_percent.unwrap_or_default(),
                coupon.coupon_name
            )
        })
        .unwrap_or_default();
    format!("${discounted}{frequency}{coupon}")
}
