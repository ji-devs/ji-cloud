use std::rc::Rc;

use dominator::{clone, events, html, with_node, Dom};
use utils::unwrap::UnwrapJiExt;
use web_sys::HtmlElement;

use crate::overlay::handle::OverlayHandle;

use super::state::Dialog;

impl Dialog {
    pub fn render(get_dom: impl Fn() -> Dom + 'static, on_close: Option<Box<dyn Fn()>>) -> Dom {
        let on_close = Rc::new(on_close);

        html!("empty-fragment" => HtmlElement, {
            .apply(OverlayHandle::lifecycle(
                move || {
                    html!("overlay-content", {
                        .property("flowContentAnchor", "mm")
                        .property("contentAnchor", "mm")
                        .child(html!("dialog-backdrop" => HtmlElement, {
                            .with_node!(elem => {
                                .event(clone!(on_close => move |e: events::Click| {
                                    let target = e.dyn_target().expect_ji("Is target really not an element??");
                                    if elem == target {
                                        if let Some(on_close) = &*on_close {
                                            on_close()
                                        }
                                    }
                                }))
                                .child(get_dom())
                            })
                        }))
                    })
                }
            ))
        })
    }
}
