use std::marker::PhantomData;

// copy of frontend/apps/crates/entry/community/src/member_details/component.rs, both should go into a shared crate
use dominator::{html, shadow_root, Dom, DomBuilder, ShadowRootMode};
use wasm_bindgen::JsCast;
use web_sys::{ShadowRoot, HtmlElement};

// using a T to get around orphan rules when implementing Component in dependents this crate,
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

    // fn apply_on_host<A>(&self, host: DomBuilder<A>) -> DomBuilder<A>
    //     where A: JsCast
    // {
    fn apply_on_host(&self, host: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {
        host
    }

    fn render_with_apply_host(&self, apply: impl Fn(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>) -> Dom {
        html!(Self::ROOT_ELEMENT, {
            .apply(|dom| {
                self.apply_on_host(dom)
            })
            .apply(|dom| {
                // self.(dom__)
                (apply)(dom)
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

    fn render_with_apply_shadow(&self, apply: impl Fn(DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot>) -> Dom {
        html!(Self::ROOT_ELEMENT, {
            .apply(|dom| {
                self.apply_on_host(dom)
            })
            .shadow_root!(Self::SHADOW_ROOT_MODE => {
                .child(html!("style", {
                    .text(Self::styles())
                }))
                .apply(|dom| {
                    (apply)(dom)
                })
                .apply(|dom| {
                    self.dom(dom)
                })
            })
        })
    }
}



// pub struct Half<A, C, T>
//     where
//         A: JsCast,
//         C: Component<T>

// {
//     dom: DomBuilder<A>,
//     p: C,
//     _t: PhantomData<T>
// }

// impl<A, C, T> Half<A, C, T>
//     where
//         A: JsCast,
//         C: Component<T>
// {
//     // fn apply_on_host<A>(&self, host: DomBuilder<A>) -> DomBuilder<A>
//     //     where A: JsCast
//     // {
//     pub fn apply(self, apply: impl Fn(DomBuilder<A>) -> DomBuilder<A>) -> Self {
//         Self {
//             p: self.p,
//             dom: (apply)(self.dom),
//             _t: PhantomData,
//         }
//     }

//     fn render(&self) -> Dom {
//         // html!(Self::ROOT_ELEMENT, {
//         //     .apply(|dom| {
//         //         self.p.apply_on_host(dom)
//         //     })
//         //     .shadow_root!(Self::SHADOW_ROOT_MODE => {
//         //         .child(html!("style", {
//         //             .text(Self::styles())
//         //         }))
//         //         .apply(|dom| {
//         //             self.p.dom(dom)
//         //         })
//         //     })
//         // })
//     }
// }
