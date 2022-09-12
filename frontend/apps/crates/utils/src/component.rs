// copy of frontend/apps/crates/entry/community/src/member_details/component.rs, both should go into a shared crate
use dominator::{html, shadow_root, Dom, DomBuilder, ShadowRootMode};
use web_sys::ShadowRoot;

// using a T to get around orphan rules when implementing Component in dependents this crate,
// not entirely sure that this is the right way of doing things though
pub trait Component<T> {
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
