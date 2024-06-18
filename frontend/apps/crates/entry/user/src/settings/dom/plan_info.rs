use components::confirm;
use dominator::{clone, html, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::billing::{
    AmountInCents, AppliedCoupon, BillingInterval, PaymentMethodType, PaymentNetwork, PlanType,
    SubscriptionTier,
};
use std::rc::Rc;
use strum::IntoEnumIterator;
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
            html!("div", {
                .prop("slot", "plan-type")
                .style("display", "flex")
                .child(html!("p", {
                    .text_signal(plan_type_signal().map(|plan| {
                        plan.map(|plan| {
                            plan.display_name()
                        }).unwrap_or_default()
                    }))
                }))
                .child_signal(plan_type_signal().map(clone!(plan_info => move |_| {
                    if plan_info.status.is_paused() {
                        Some(html!("span", {
                            .style("align-self", "center")
                            .style("margin-left", "10rem")
                            .child(html!("pill-close", {
                                .prop("label", "Paused")
                            }))
                        }))
                    } else {
                        None
                    }
                })))
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
                .child_signal(plan_type_signal().map(clone!(state, plan_info => move |plan_type| {
                    if !plan_info.status.is_active() {
                        return None;
                    }

                    let frequency = plan_type?.billing_interval();
                    match frequency {
                        BillingInterval::Monthly => {
                            Some(html!("button-rect", {
                                .prop("type", "filled")
                                .prop("color", "blue")
                                .text(match plan_type {
                                    Some(PlanType::SchoolLevel1Monthly) => "Get 1 month FREE by switching to annual billing",
                                    _ => "Get extra saving by switching to annual billing"
                                })
                                .event(clone!(state, plan_info => move|_ :events::Click| {
                                    spawn_local(clone!(state, plan_info => async move {
                                        let plan_type: PlanType = plan_type.unwrap_ji();
                                        let new_plan_type = plan_type.monthly_to_annual();
                                        let new_price = discounted_price(new_plan_type, &plan_info);
                                        let message = format!("You will be charged {new_price} per year. A renewal reminder will be sent 30 days before the end of your subscription.");

                                        let confirmed = confirm::Confirm {
                                            title: "Switch to Annual Billing".to_string(),
                                            message,
                                            confirm_text: "Confirm".to_string(),
                                            cancel_text: "Cancel".to_string()
                                        }.confirm().await;
                                        if confirmed {
                                            state.change_to(new_plan_type)
                                        }
                                    }));
                                }))
                            }))
                        },
                        BillingInterval::Annually => None,
                    }
                })))
                .child_signal(plan_type_signal().map(clone!(state, plan_info => move |plan_type| {
                    if !plan_info.status.is_active() {
                        return None;
                    }

                    let plan_type: PlanType = plan_type?;
                    let subscription_tier = plan_type.subscription_tier();
                    if plan_type.is_individual_plan() {
                        match subscription_tier {
                            SubscriptionTier::Basic => {
                                let new_plan_type = plan_type.basic_to_pro();
                                let billing_interval = plan_type.billing_interval();
                                Some(html!("button-rect", {
                                    .prop("type", "filled")
                                    .prop("color", "blue")
                                    .text(&format!("Switch to Pro {billing_interval}"))
                                    .event(clone!(state, plan_info => move|_ :events::Click| {
                                        spawn_local(clone!(state, plan_info => async move {
                                            let charge_interval = match billing_interval {
                                                BillingInterval::Monthly => "month",
                                                BillingInterval::Annually => "year",
                                            };
                                            let renewal_message = match billing_interval {
                                                BillingInterval::Annually =>  " A renewal reminder will be sent 30 days before the end of your subscription.",
                                                BillingInterval::Monthly => " You can pause your subscription or cancel at any time.",
                                            };
                                            let new_price = discounted_price(new_plan_type, &plan_info);
                                            let message = format!("You will be charged {new_price} per {charge_interval}.{renewal_message}");

                                            let confirmed = confirm::Confirm {
                                                title: "Switch to the Pro plan".to_string(),
                                                message,
                                                confirm_text: "Confirm".to_string(),
                                                cancel_text: "Cancel".to_string()
                                            }.confirm().await;
                                            if confirmed {
                                                state.change_to(new_plan_type);
                                            }
                                        }));
                                    }))
                                }))
                            },
                            SubscriptionTier::Pro => None,
                        }
                    } else {
                        match plan_type {
                            PlanType::SchoolUnlimitedMonthly | PlanType::SchoolUnlimitedAnnually => None,
                            _ => {
                                let plan_type_signal = Mutable::new(None::<PlanType>);
                                let plan_types = PlanType::iter().filter(|p| {
                                    p.can_upgrade_from_same_interval(&plan_type)
                                }).map(|p| html!("input-select-option", {
                                    .text(p.user_display_name())
                                    .prop("value", p.as_str())
                                    .prop_signal("selected", plan_type_signal.signal_cloned().map(move |current| match current {
                                        None => false,
                                        Some(current) => current == p,
                                    }))
                                    .event(clone!(state, plan_info, plan_type_signal => move |e: events::CustomSelectedChange| {
                                        if e.selected() {
                                            plan_type_signal.set(Some(p));
                                            spawn_local(clone!(state, plan_info, plan_type_signal => async move {
                                                let billing_interval = plan_type.billing_interval();
                                                let charge_interval = match billing_interval {
                                                    BillingInterval::Monthly => "month",
                                                    BillingInterval::Annually => "year",
                                                };

                                                let renewal_message = match billing_interval {
                                                    BillingInterval::Annually =>  " A renewal reminder will be sent 30 days before the end of your subscription.",
                                                    BillingInterval::Monthly => " You can pause your subscription or cancel at any time.",
                                                };
                                                let new_price = discounted_price(p, &plan_info);

                                                let message = format!("You will be charged {new_price} per {charge_interval}.{renewal_message}");

                                                let confirmed = confirm::Confirm {
                                                    title: format!("Upgrade to {}", p.user_display_name()),
                                                    message,
                                                    confirm_text: "Confirm".to_string(),
                                                    cancel_text: "Cancel".to_string()
                                                }.confirm().await;
                                                if confirmed {
                                                    state.change_to(p);
                                                } else {
                                                    plan_type_signal.set(None);
                                                }
                                            }));
                                        }
                                    }))
                                })).collect::<Vec<_>>();
                                Some(html!("input-select", {
                                    .style("min-width", "200rem")
                                    .prop("label", "Upgrade to")
                                    .prop("multiple", false)
                                    .prop_signal("value", plan_type_signal.signal_cloned().map(|p| p.map_or("", |p| p.user_display_name())))
                                    .children(plan_types)
                                }))
                            }
                        }
                    }
                })))
                .child_signal(plan_type_signal().map(clone!(state, plan_info => move |plan_type| {
                    if plan_info.is_trial {
                        return None;
                    }

                    if let Some(plan_type) = plan_type {
                        if let BillingInterval::Annually = plan_type.billing_interval() {
                            return None;
                        }
                    }
                    if !plan_info.status.is_active() && !plan_info.status.is_paused() {
                        return None;
                    }

                    Some(html!("button-rect", {
                        .prop("type", "filled")
                        .prop("color", "blue")
                        .text(match plan_info.status.is_paused() {
                            true => "Resume subscription",
                            false => "Pause subscription"
                        })
                        .event(clone!(state, plan_info => move|_ :events::Click| {
                            spawn_local(clone!(state, plan_info => async move {
                                let (title, message, confirm_text) = if plan_info.status.is_paused() {
                                    (
                                        "Resume subscription".to_string(),
                                        "Are you sure you want to resume your subscription?".to_string(),
                                        "Resume".to_string(),
                                    )
                                } else {
                                    (
                                        "Pause subscription".to_string(),
                                        "Are you sure you want to pause your subscription?".to_string(),
                                        "Pause".to_string(),
                                    )
                                };
                                let confirmed = confirm::Confirm {
                                    title,
                                    message,
                                    confirm_text,
                                    cancel_text: "Cancel".to_string()
                                }.confirm().await;
                                if confirmed {
                                    state.set_paused(!plan_info.status.is_paused())
                                }
                            }));
                        }))
                    }))
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

fn discounted_price(plan_type: PlanType, plan_info: &Rc<PlanSectionInfo>) -> String {
    let discount_percent = match plan_info.coupon {
        Some(AppliedCoupon { coupon_percent, .. }) => match coupon_percent {
            Some(percent) => 1.0 - f64::from(percent),
            None => 1.0,
        },
        None => 1.0,
    };
    let new_plan_price = (plan_type.plan_price() as f64 * discount_percent) as u32; // This should still be cents
    number_as_price(new_plan_price)
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
