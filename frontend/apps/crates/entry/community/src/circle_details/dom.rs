use std::rc::Rc;

use super::{
    callbacks::EditCirclesCallbacks, edit_about::EditAbout, edit_image::EditImage,
    edit_name::EditName, ActivePopup, CircleDetails,
};
use components::dialog::Dialog;
use dominator::{clone, html, Dom};
use futures_signals::{signal::SignalExt, signal_vec::SignalVecExt};
use itertools::Itertools;
use shared::{
    domain::{
        circle::Circle,
        user::{public_user::PublicUser, UserId},
    },
    media::MediaLibrary,
};
use utils::{
    component::Component,
    events,
    languages::Language,
    prelude::get_user_id,
    routes::{CommunityMembersRoute, CommunityRoute, Route},
    unwrap::UnwrapJiExt,
};
use wasm_bindgen::JsValue;

// const STR_CONTACT_ADMIN: &str = "Contact admin";
// const STR_INVITE: &str = "Invite";
const STR_MEMBER: &str = "Member";
const STR_DELETE_CIRCLE: &str = "Delete circle";
const STR_JOIN: &str = "Join";
const STR_SEARCH_MEMBER: &str = "Search member";

fn user_id_is_circle_author(user_id: &Option<UserId>, circle: &Circle) -> bool {
    match user_id {
        Some(user_id) => user_id == &circle.created_by,
        None => false,
    }
}

impl CircleDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_data();

        html!("empty-fragment", {
            .child_signal(state.circle.signal_ref(clone!(state => move |circle| {
                circle.as_ref().map(|circle| {
                    html!("community-circle-details", {
                        .prop("name", &circle.display_name)
                        .prop("description", &circle.description)
                        .prop("memberCount", circle.member_count)
                        .children(&mut [
                            html!("img-ji", {
                                .prop("slot", "image")
                                .prop("lib", MediaLibrary::User.to_str())
                                .prop("id", &circle.image.0.to_string())
                            }),
                            // html!("button-rect", {
                            //     .prop("slot", "actions")
                            //     .prop("kind", "outline")
                            //     .prop("size", "small")
                            //     .prop("color", "blue")
                            //     .text(STR_CONTACT_ADMIN)
                            // }),
                            // html!("button-rect", {
                            //     .prop("slot", "actions")
                            //     .prop("kind", "outline")
                            //     .prop("size", "small")
                            //     .prop("color", "blue")
                            //     .text(STR_INVITE)
                            // }),
                            // member-images
                            html!("input-search", {
                                .prop("slot", "member-search")
                                .prop("placeholder", STR_SEARCH_MEMBER)
                            }),
                        ])
                        .apply_if(
                            user_id_is_circle_author(&get_user_id(), circle),
                            clone!(state => move |dom| {
                                dom.children(&mut [
                                    html!("button-rect", {
                                        .prop("slot", "actions")
                                        .prop("kind", "outline")
                                        .prop("size", "small")
                                        .prop("color", "red")
                                        .text(STR_DELETE_CIRCLE)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.delete_circle();
                                        }))
                                    }),
                                    html!("fa-button", {
                                        .prop("slot", "edit-image")
                                        .prop("icon", "fa-light fa-pen")
                                        .text("about")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.active_popup.set(Some(ActivePopup::Image))
                                        }))
                                    }),
                                    html!("fa-button", {
                                        .prop("slot", "edit-name")
                                        .prop("icon", "fa-light fa-pen")
                                        .text("about")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.active_popup.set(Some(ActivePopup::Name))
                                        }))
                                    }),
                                    html!("fa-button", {
                                        .prop("slot", "edit-about")
                                        .prop("icon", "fa-light fa-pen")
                                        .prop("dir", "auto")
                                        .text("about")
                                        .event(clone!(state => move |_: events::Click| {
                                            state.active_popup.set(Some(ActivePopup::About))
                                        }))
                                    }),
                                ])
                            })
                        )
                        .child_signal(state.community_state.user.signal_ref(clone!(state => move |user| {
                            let is_member = match user {
                                Some(user) => user.circles.iter().any(|circle| circle == &state.circle_id),
                                None => false,
                            };
                            Some(match is_member {
                                true => {
                                    html!("button-rect", {
                                        .prop("slot", "actions")
                                        .prop("kind", "outline")
                                        .prop("size", "small")
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
                                        .prop("slot", "actions")
                                        .prop("kind", "outline")
                                        .prop("size", "small")
                                        .prop("color", "blue")
                                        .text(STR_JOIN)
                                        .event(clone!(state => move |_: events::Click| {
                                            state.join_circle();
                                        }))
                                    })
                                },
                            })
                        })))
                        .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
                            state.render_member(&member)
                        })))
                    })
                })
            })))

            .child_signal(state.active_popup.signal().map(clone!(state => move |active_popup| {
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
            })))
        })
    }

    fn render_member(self: &Rc<Self>, member: &PublicUser) -> Dom {
        html!("community-list-member", {
            .prop("slot", "members")
            .prop("name", &format!("{} {}", member.given_name, member.family_name))
            // .prop("city", "New York")
            // .prop("state", "NY")
            .apply(|mut dom| {
                if let Some(languages_spoken) = &member.languages_spoken {
                    if languages_spoken.len() > 0 {
                        let languages = languages_spoken.iter().map(|l| Language::code_to_display_name(l)).join(", ");
                        dom = dom.prop("language", languages);
                    };
                }
                dom
            })
            .apply(move |dom| dominator::on_click_go_to_url!(dom, {
                Route::Community(CommunityRoute::Members(CommunityMembersRoute::Member(member.id))).to_string()
            }))
            .child(html!("profile-image", {
                .prop("slot", "img")
                .prop("imageId", {
                    match &member.profile_image {
                        Some(image_id) => JsValue::from_str(&image_id.0.to_string()),
                        None => JsValue::UNDEFINED,
                    }
                })
            }))
        })
    }
}
