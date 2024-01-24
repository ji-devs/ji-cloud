use discard::Discard;
use dominator::{self, clone, html, shadow_root, DomHandle, ShadowRootMode};
use std::{cell::RefCell, rc::Rc};
use utils::callback_future::CallbackFuture;
use utils::{events, unwrap::UnwrapJiExt};
use web_sys::HtmlDialogElement;

pub struct Confirm {
    pub title: String,
    pub message: String,
    pub confirm_text: String,
    pub cancel_text: String,
}

impl Default for Confirm {
    fn default() -> Self {
        Self {
            title: String::from("Warning"),
            message: String::from("Are you sure?"),
            confirm_text: String::from("Yes"),
            cancel_text: String::from("Cancel"),
        }
    }
}

impl Confirm {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            ..Default::default()
        }
    }

    pub fn confirm(self) -> CallbackFuture<bool> {
        CallbackFuture::new(Box::new(move |resolve| {
            let resolve = Rc::new(RefCell::new(Some(resolve)));
            let dom_handle: Rc<RefCell<Option<DomHandle>>> = Rc::new(RefCell::new(None));

            *dom_handle.borrow_mut() = Some(dominator::append_dom(
                &dominator::get_id("root"),
                html!("div", {
                    .shadow_root!(ShadowRootMode::Open => {
                        .child(html!("style", {
                            .text(include_str!("./styles.css"))
                        }))
                        .child(html!("dialog" => HtmlDialogElement, {
                            .after_inserted(|dialog: HtmlDialogElement| {
                                let _ = dialog.show_modal();
                            })
                            .child(html!("popup-body", {
                                .child(html!("fa-button", {
                                    .prop("slot", "close")
                                    .prop("icon", "fa-regular fa-xmark")
                                    .event(clone!(resolve, dom_handle => move |_: events::Click| {
                                        let resolve = resolve.borrow_mut().take().unwrap_ji();
                                        (resolve)(false);
                                        let dom_handle = dom_handle.borrow_mut().take().unwrap_ji();
                                        dom_handle.discard();
                                    }))
                                }))
                                .child(html!("h3", {
                                    .prop("slot", "heading")
                                    .text(&self.title)
                                }))
                                .child(html!("div", {
                                    .prop("slot", "body")
                                    .children(&mut [
                                        html!("p", {
                                            .text(&self.message)
                                        }),
                                        html!("button-rect", {
                                            .prop("size", "regular")
                                            .prop("color", "red")
                                            .prop("kind", "outline")
                                            .text(&self.cancel_text)
                                            .event(clone!(resolve, dom_handle => move |_: events::Click| {
                                                let resolve = resolve.borrow_mut().take().unwrap_ji();
                                                (resolve)(false);
                                                let dom_handle = dom_handle.borrow_mut().take().unwrap_ji();
                                                dom_handle.discard();
                                            }))
                                        }),
                                        html!("button-rect", {
                                            .prop("size", "regular")
                                            .prop("color", "red")
                                            .prop("kind", "filled")
                                            .text(&self.confirm_text)
                                            .event(clone!(resolve, dom_handle => move |_: events::Click| {
                                                let resolve_fn = resolve.borrow_mut().take().unwrap_ji();
                                                (resolve_fn)(true);
                                                let dom_handle = dom_handle.borrow_mut().take().unwrap_ji();
                                                dom_handle.discard();
                                            }))
                                        }),
                                    ])
                                }))
                            }))

                        }))
                    })
                }),
            ));
        }))
    }
}
