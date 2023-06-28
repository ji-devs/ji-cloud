use std::rc::Rc;

use super::{
    callbacks::EditCirclesCallbacks, edit_about::EditAbout, edit_image::EditImage,
    edit_name::EditName, ActivePopup, CircleDetails,
};
use crate::member_card::MemberCard;
use components::dialog::Dialog;
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use shared::{
    api::endpoints::circle::RemoveMember,
    domain::{
        circle::{Circle, CircleRemoveMemberPath},
        user::{public_user::PublicUser, UserId},
    },
    media::MediaLibrary,
};
use utils::{
    clipboard,
    component::Component,
    events,
    prelude::{get_user_id, ApiEndpointExt},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::ShadowRoot;

// const STR_CONTACT_ADMIN: &str = "Contact admin";
const STR_INVITE: &str = "Copy link to invite";
const STR_COPIED: &str = "Invite link copied!";
const STR_MEMBER: &str = "Member";
const STR_DELETE_CIRCLE: &str = "Delete circle";
const STR_JOIN: &str = "Join";
// const STR_SEARCH_MEMBER: &str = "Search member";

impl Component<CircleDetails> for Rc<CircleDetails> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        state.load_data();
        dom.children_signal_vec(
            state
                .circle
                .signal_ref(clone!(state => move |circle| {
                    circle.as_ref().map(clone!(state => move |circle| {
                        let current_user_admin = is_circle_admin(circle, get_user_id());
                        vec![
                            state.render_header(circle, current_user_admin),
                            state.render_about(circle, current_user_admin),
                            state.render_members(circle, current_user_admin),
                        ]
                    })).unwrap_or_default()
                }))
                .to_signal_vec(),
        )
        .apply(|dom| state.popups_mixin(dom))
        // .children(&mut [
        //     html!("button-rect", {
        //         .prop("slot", "actions")
        //         .prop("kind", "outline")
        //         .prop("color", "blue")
        //         .text(STR_CONTACT_ADMIN)
        //     }),
        //     html!("input-search", {
        //         .prop("slot", "member-search")
        //         .prop("placeholder", STR_SEARCH_MEMBER)
        //     }),
        // ])
    }
}

impl CircleDetails {
    fn render_header(self: &Rc<Self>, circle: &Circle, current_user_admin: bool) -> Dom {
        let state = self;
        html!("div", {
            .class("header")
            .child(html!("img-ji", {
                .prop("slot", "image")
                .prop("lib", MediaLibrary::User.to_str())
                .prop("id", &circle.image.0.to_string())
            }))
            .apply_if(current_user_admin, clone!(state => move |dom| {
                dom.child(html!("fa-button", {
                        .prop("slot", "edit-image")
                        .prop("icon", "fa-light fa-pen")
                        .class("edit-image")
                        .event(clone!(state => move |_: events::Click| {
                            state.active_popup.set(Some(ActivePopup::Image))
                        }))
                    })
                )})
            )
            .child(html!("div", {
                .class("name")
                .child(html!("h1", {
                    .text(&circle.display_name)
                }))
                .apply_if(current_user_admin, clone!(state => move |dom| {
                    dom.child(
                        html!("fa-button", {
                            .prop("slot", "edit-name")
                            .prop("icon", "fa-light fa-pen")
                            .class("edit-name")
                            .event(clone!(state => move |_: events::Click| {
                                state.active_popup.set(Some(ActivePopup::Name))
                            }))
                        }),
                    )})
                )
            }))
            .child(html!("hr"))
            .child(html!("div", {
                .class("actions")
                .child(html!("button-rect", {
                    .class("action")
                    .prop("kind", "text")
                    .prop("color", "blue")
                    .text_signal(state.url_copied.signal().map(|url_copied| match url_copied {
                        false => STR_INVITE,
                        true => STR_COPIED,
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        fn get_current_url() -> Option<String> {
                            Some(web_sys::window()?
                                .location()
                                .href().ok()?)
                        }
                        let circle_url = get_current_url().unwrap_ji();
                        clipboard::write_text(&circle_url);
                        state.url_copied.set(true);
                    }))
                }))
                .apply_if(current_user_admin, clone!(state => move |dom| {
                    dom.child(html!("button-rect", {
                        .class("action")
                        .prop("kind", "outline")
                        .prop("color", "red")
                        .text(STR_DELETE_CIRCLE)
                        .event(clone!(state => move |_: events::Click| {
                            state.delete_circle();
                        }))
                    }))
                }))
                .apply_if(!current_user_admin, clone!(state => move |dom| {
                    dom.child_signal(state.joined.signal_ref(clone!(state => move |joined| {
                        joined.map(|joined| match joined {
                            true => {
                                html!("button-rect", {
                                    .class("action")
                                    .prop("kind", "outline")
                                    .prop("color", "green")
                                    .child(html!("fa-icon", {
                                        .prop("icon", "fa-solid fa-check")
                                    }))
                                    .text(STR_MEMBER)
                                    .event(clone!(state => move |_: events::Click| {
                                        state.leave_circle();
                                    }))
                                })
                            },
                            false => {
                                html!("button-rect", {
                                    .class("action")
                                    .prop("kind", "outline")
                                    .prop("color", "blue")
                                    .text(STR_JOIN)
                                    .event(clone!(state => move |_: events::Click| {
                                        state.join_circle();
                                    }))
                                })
                            },
                        })
                    })))
                }))
            }))
        })
    }

    fn render_about(self: &Rc<Self>, circle: &Circle, current_user_admin: bool) -> Dom {
        let state = self;
        html!("section", {
            .class("about-section")
            .child(html!("div", {
                .class("section-header")
                .child(html!("h3", {
                    .text("About")
                }))
                .apply_if(
                    current_user_admin,
                    clone!(state => move |dom| {
                        dom.child(html!("fa-button", {
                            .prop("slot", "edit-about")
                            .prop("icon", "fa-light fa-pen")
                            .prop("dir", "auto")
                            .text("about")
                            .event(clone!(state => move |_: events::Click| {
                                state.active_popup.set(Some(ActivePopup::About))
                            }))
                        }))
                    })
                )
            }))
            .child(html!("p", {
                .text(&circle.description)
                .attr("dir", "auto")
            }))
        })
    }

    fn render_members(self: &Rc<Self>, circle: &Circle, current_user_admin: bool) -> Dom {
        let state = self;
        let circle = circle.clone();
        let current_user_id = get_user_id();
        html!("section", {
            .class("members-section")
            .child(html!("h3", {
                .text("Members")
            }))
            .child(html!("div", {
                .class("members")
                .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
                    state.render_member(&member, &circle, current_user_admin, current_user_id)
                })))
            }))
        })
    }

    fn render_member(
        self: &Rc<Self>,
        member: &PublicUser,
        circle: &Circle,
        current_user_admin: bool,
        current_user_id: Option<UserId>,
    ) -> Dom {
        let state = self;
        let member_id = member.id;
        let member_is_admin = is_circle_admin(circle, Some(member_id));
        let menu = match current_user_admin && !member_is_admin {
            false => None,
            true => Some(html!("menu-kebab", {
                .prop("slot", "menu")
                .child(html!("menu-line", {
                    .prop("icon", "delete")
                    .event(clone!(state => move |_: events::Click| {
                        spawn_local(clone!(state => async move {
                            RemoveMember::api_with_auth_empty(
                                CircleRemoveMemberPath(state.circle_id, member_id),
                                None,
                            ).await.unwrap_ji();
                            let mut members = state.members.lock_mut();
                            if let Some(index) = members.iter().position(|member| member.id == member_id) {
                                members.remove(index);
                            }
                            if let Some(circle) = state.circle.lock_mut().as_mut() {
                                circle.member_count -= 1;
                            }
                        }));
                    }))
                }))
            })),
        };
        MemberCard {
            member,
            admin_tag: member_is_admin,
            menu,
            current_user_id,
        }
        .render()
    }

    fn popups_mixin(self: &Rc<Self>, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child_signal(
            state
                .active_popup
                .signal()
                .map(clone!(state => move |active_popup| {
                    active_popup.map(clone!(state => move |active_popup| {
                        Dialog::render(
                            clone!(state => move || {
                                let callbacks = EditCirclesCallbacks {
                                    save_changes: Box::new(clone!(state => move|circle| {
                                        state.save_circle_changes(circle);
                                    })),
                                    close: Box::new(clone!(state => move || {
                                        state.active_popup.set(None);
                                    }))
                                };
                                match active_popup {
                                    ActivePopup::About => {
                                        EditAbout::new(
                                            state.circle.get_cloned().unwrap_ji(),
                                            callbacks
                                        ).render()
                                    },
                                    ActivePopup::Name => {
                                        EditName::new(
                                            state.circle.get_cloned().unwrap_ji(),
                                            callbacks
                                        ).render()
                                    },
                                    ActivePopup::Image => {
                                        EditImage::new(
                                            state.circle.get_cloned().unwrap_ji(),
                                            callbacks
                                        ).render()
                                    },
                                }
                            }),
                            Some(Box::new(clone!(state => move || {
                                state.active_popup.set(None);
                            })))
                        )
                    }))
                })),
        )
    }
}

fn is_circle_admin(circle: &Circle, user_id: Option<UserId>) -> bool {
    matches!(
        (circle, user_id),
        (circle, Some(user_id)) if circle.created_by == user_id
    )
}
