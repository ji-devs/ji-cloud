use components::confirm;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::billing::{
    AmountInCents, AppliedCoupon, BillingInterval, PaymentMethodType, PaymentNetwork, PlanType,
};
use std::rc::Rc;
use utils::{events, js_object, prelude::plan_type_signal, unwrap::UnwrapJiExt};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

use crate::settings::state::{PlanSectionInfo, SettingsPage};

const STR_EXPIRED_ON: &str = "Expired on";
const STR_TRIAL_ENDS_ON: &str = "Trial ends on";
const STR_RENEWS_ON: &str = "Renews on";
const STR_EXPIRES_ON: &str = "Expires on";

const DATE_FORMAT: &str = "%h %e, %Y";

impl SettingsPage {
    pub(super) fn render_plan_section(
        self: &Rc<Self>,
        plan_info: &Rc<PlanSectionInfo>,
    ) -> Vec<Dom> {
        let state = self;
        let auto_renew = plan_info.auto_renew.read_only();

        self.load_portal_link();

        vec![
            html!("div", {
                .prop("slot", "portal-link")
                .child_signal(self.portal_link.signal_cloned().map(|link| {
                    link.as_ref().map(|link| {
                        html!("button-rect", {
                            .prop("href", link)
                            .prop("target", "_blank")
                            .text("Customer portal")
                        })
                    })
                }))
            }),
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
                        let frequency = plan_type.billing_interval();
                        price_string(&plan_info.price, &plan_info.coupon, frequency)
                    }).unwrap_or_default()
                })))
            }),
            html!("p", {
                .prop("slot", "plan-renews-on")
                .text(&plan_info.current_period_end.format(DATE_FORMAT).to_string())
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
                    let frequency = plan_type?.billing_interval();
                    match frequency {
                        BillingInterval::Monthly => {
                            Some(html!("button-rect", {
                                .prop("slot", "change-to-annual")
                                .prop("type", "filled")
                                .prop("color", "blue")
                                .text(match plan_type {
                                    Some(PlanType::SchoolLevel1Monthly) => "Get 1 month FREE by switching to annual billing",
                                    _ => "Get 2 months FREE by switching to annual billing"
                                })
                                .event(clone!(state => move|_ :events::Click| {
                                    spawn_local(clone!(state => async move {
                                        let new_price = plan_type.unwrap().monthly_to_annual().plan_price();
                                        let new_price = number_as_price(new_price);
                                        let message = format!("You will be charged {new_price} per year. A renewal reminder will be sent 30 days before the end of your subscription.");

                                        let confirmed = confirm::Confirm {
                                            title: "Switch to Annual Billing".to_string(),
                                            message,
                                            confirm_text: "Confirm".to_string(),
                                            cancel_text: "Cancel".to_string()
                                        }.confirm().await;
                                        if confirmed {
                                            state.change_to_annual_billing()
                                        }
                                    }));
                                }))
                            }))
                        },
                        BillingInterval::Annually => None,
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
    frequency: BillingInterval,
) -> String {
    let discounted = AmountInCents::from(
        coupon
            .as_ref()
            .and_then(|coupon| coupon.coupon_percent)
            .map_or(price.inner() as f64, |percent| {
                let price = price.inner() as f64;
                price - (price * f64::from(percent))
            }) as i64,
    );

    let frequency = match frequency {
        BillingInterval::Annually => " per year",
        BillingInterval::Monthly => " per month",
    };

    let coupon = coupon
        .as_ref()
        .map(|coupon| {
            let applied_to = coupon
                .coupon_to
                .map(|to| format!(" until {}", to.date_naive().format(DATE_FORMAT)))
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

/// Move to utils?
fn number_as_price(cents: u32) -> String {
    let locales = js_sys::Array::of1(&JsValue::from("en-US"));
    let options = js_object!({
        "style": "currency",
        "currency": "usd",
    });
    let options = js_sys::Object::try_from(&options).unwrap();
    let price = js_sys::Intl::NumberFormat::new(&locales, &options);
    let cents = JsValue::from_f64(cents as f64 / 100.00);
    price
        .format()
        .call1(&price, &cents)
        .unwrap_ji()
        .as_string()
        .unwrap_ji()
}
