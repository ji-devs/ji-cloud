use dominator::{html, shadow_root, Dom, DomBuilder, ShadowRootMode};
use web_sys::ShadowRoot;

pub trait Component {
    const ROOT_ELEMENT: &'static str = "div";
    const SHADOW_ROOT_MODE: ShadowRootMode = ShadowRootMode::Open;

    fn styles() -> &'static str;

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot>;

    fn render(&self) -> Dom {
        html!(Self::ROOT_ELEMENT, {
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
}
