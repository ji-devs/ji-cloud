use gloo_utils::format::JsValueSerdeExt;
use js_sys::Reflect;
use serde_json::json;
use utils::unwrap::UnwrapJiExt;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "@stripe/stripe-js")]
extern "C" {
    /// returns Stripe
    #[wasm_bindgen]
    async fn loadStripe(publishableKey: &str) -> JsValue;

    #[wasm_bindgen(js_name = "Stripe")]
    type JsStripe;

    #[wasm_bindgen(method)]
    fn elements(this: &JsStripe, options: JsValue) -> StripeElements;

    #[wasm_bindgen(method)]
    async fn confirmSetup(this: &JsStripe, options: JsValue) -> JsValue;

    #[wasm_bindgen(js_name = "StripeElements")]
    type StripeElements;

    #[wasm_bindgen(method)]
    fn create(this: &StripeElements, elementType: &str, options: JsValue) -> StripePaymentElement;

    #[wasm_bindgen(js_name = "StripePaymentElement")]
    type StripePaymentElement;

    #[wasm_bindgen(method)]
    fn mount(this: &StripePaymentElement, domElement: HtmlElement);
}

// TODO: move to utils?
macro_rules! js_object {
    ($($value:tt)+) => {
        <wasm_bindgen::JsValue as JsValueSerdeExt>::from_serde(&json!($($value)+)).unwrap_ji()
    };
}

pub struct Stripe {
    js_stripe: JsStripe,
    elements: StripeElements,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "release")] {
        const PUBLISHABLE_KEY: &str = "pk_live_lJqRutp8QHyjasfedIVi3GrE";
    } else {
        const PUBLISHABLE_KEY: &str = "pk_test_cSUzMZSsUmmgzHRXPNEq5YOm";
    }
}

impl Stripe {
    pub async fn new(el: HtmlElement, client_secret: String) -> Self {
        let js_stripe: JsStripe = loadStripe(PUBLISHABLE_KEY).await.into();
        let elements = js_stripe.elements(js_object!({
            "clientSecret": client_secret,
            "appearance": {
                "theme": "flat",
            },
        }));
        let payment_element = elements.create(
            "payment",
            js_object!({
                "layout": {
                    "type": "tabs",
                    "defaultCollapsed": false
                },
                "paymentMethodOrder": ["card", "apple_pay", "google_pay", "link"],
            }),
        );
        payment_element.mount(el);
        Self {
            js_stripe,
            elements,
        }
    }

    pub async fn submit(&self, redirect_url: &str) {
        let options = js_object!({
            "confirmParams": {
                "return_url": redirect_url,
            }
        });
        // manually set elements field
        Reflect::set(
            &options,
            &JsValue::from_str("elements"),
            &(&self.elements).into(),
        )
        .unwrap_ji();
        self.js_stripe.confirmSetup(options).await;
    }
}
