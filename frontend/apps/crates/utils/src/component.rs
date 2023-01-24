/*
    all this trait does it add a shadow dom with styles, maybe just add a macro that does this similar to `html!`
    ```
        host!("./styles.css", {
            .child(...)
        })
    ```
*/

use dominator::{html, shadow_root, Dom, DomBuilder, ShadowRootMode};
use web_sys::{HtmlElement, ShadowRoot};

// using a T to get around orphan rules when implementing Component in dependents of this crate,
// not entirely sure that this is the right way of doing things though
pub trait Component<T> {
    const ROOT_ELEMENT: &'static str = "div";
    const SHADOW_ROOT_MODE: ShadowRootMode = ShadowRootMode::Open;

    fn styles() -> &'static str;

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot>;

    fn render(&self) -> Dom {
        html!(Self::ROOT_ELEMENT, {
            .apply(|dom| {
                self.apply_on_host(dom)
            })
            .shadow_root!(Self::SHADOW_ROOT_MODE => {
                .child(html!("style", {
                    .text(Self::styles())
                }))
                .apply(|dom| {
                    self.dom(dom)
                })
            })
        })
    }

    fn apply_on_host(&self, host: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        host
    }
}
