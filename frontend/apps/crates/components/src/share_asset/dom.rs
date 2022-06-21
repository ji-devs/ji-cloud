use std::rc::Rc;

use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::SignalExt;
use js_sys::encode_uri_component;
use shared::config::JIG_PLAYER_SESSION_VALID_DURATION_SECS;
use utils::{
    clipboard, events,
    prelude::SETTINGS,
    routes::{KidsRoute, Route},
    unwrap::UnwrapJiExt,
};
use web_sys::{window, HtmlElement};

use crate::overlay::handle::OverlayHandle;

use super::state::{ActivePopup, ShareAsset};

const STR_BACK: &str = "Back";
const STR_STUDENTS_COPY_CODE_LABEL: &str = "Copy code";
const STR_STUDENTS_COPIED_CODE_LABEL: &str = "Student code copied";
const STR_STUDENTS_COPY_URL_LABEL: &str = "Copy URL";
const STR_STUDENTS_COPIED_URL_LABEL: &str = "URL copied";
const STR_EMBED_COPY_CODE_LABEL: &str = "Copy code";
const STR_EMBED_COPIED_CODE_LABEL: &str = "Embed code copied";
const STR_CLASSROOM: &str = "Share to Google Classroom";
const STR_STUDENTS_LABEL: &str = "Share with students";
const STR_EMBED_LABEL: &str = "Embed this JIG";
const STR_COPY_LABEL: &str = "Copy JIG link";
const STR_COPIED_LABEL: &str = "JIG link copied";

impl ShareAsset {
    pub fn render(self: Rc<Self>, anchor: Dom, slot: Option<&str>) -> Dom {
        let state = self;
        html!("empty-fragment" => HtmlElement, {
            .with_node!(elem => {
                .apply_if(slot.is_some(), |dom| {
                    dom.property("slot", slot.unwrap_ji())
                })
                .event(clone!(state => move |_: events::Close| {
                    state.active_popup.set(None);
                }))
                .child(html!("empty-fragment", {
                    .property("slot", "anchor")
                    .style("display", "flex")
                    .event(clone!(state => move |_: events::Click| {
                        let new_value = match &*state.active_popup.lock_ref() {
                            Some(_) => None,
                            _ => Some(ActivePopup::ShareMain),
                        };
                        state.active_popup.set(new_value);
                    }))
                    .child(anchor)
                }))
                .apply(OverlayHandle::lifecycle(
                    move || {
                        html!("overlay-content", {
                            .property("target", &elem)
                            .property("contentAnchor", "oppositeH")
                            .property("targetAnchor", "tr")
                            .event(clone!(state => move |_:events::Close| {
                                state.active_popup.set(None);
                            }))
                            .child(html!("empty-fragment", {
                                .child_signal(state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
                                    match active_popup {
                                        Some(ActivePopup::ShareMain) => {
                                            Some(state.render_share_main())
                                        },
                                        Some(ActivePopup::ShareStudents) => {
                                            Some(state.render_share_students())
                                        },
                                        Some(ActivePopup::ShareEmbed) => {
                                            Some(state.render_share_embed())
                                        },
                                        _ => None,
                                    }
                                })))
                            }))
                        })
                    }
                ))
            })
        })
    }

    fn render_share_main(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("share-jig-main", {
            .property("slot", "overlay")
            .children(&mut [
                html!("fa-button", {
                    .property("slot", "close")
                    .property("icon", "fa-light fa-xmark")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("share-jig-option", {
                    .property("kind", "students")
                    .text(STR_STUDENTS_LABEL)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareStudents));
                    }))
                }),
                html!("share-jig-option", {
                    .property("kind", "google-classroom")
                    .text(STR_CLASSROOM)
                    .event(clone!(state => move |_: events::Click| {
                        if let Some(window) = window() {
                            let share_url = format!("https://classroom.google.com/share?url={}", encode_uri_component(&state.jig_link(true)));
                            let _ = window.open_with_url_and_target(&share_url, "_blank");
                        }
                    }))
                }),
                html!("share-jig-option", {
                    .property("kind", "embed")
                    .text(STR_EMBED_LABEL)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareEmbed));
                    }))
                }),
                html!("share-jig-option", {
                    .property("kind", "copy")
                    .text_signal(state.link_copied.signal().map(|copied| {
                        match copied {
                            false => STR_COPY_LABEL,
                            true => STR_COPIED_LABEL,
                        }
                    }))
                    .event(clone!(state => move|_: events::Click| {
                        clipboard::write_text(&state.jig_link(false));
                        ShareAsset::set_copied_mutable(state.link_copied.clone());
                    }))
                }),
            ])
        })
    }

    fn render_share_students(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("share-jig-students", {
            .property("slot", "overlay")
            .property_signal("url", state.student_code.signal_cloned().map(|student_code| {
                match student_code {
                    None => String::new(),
                    Some(student_code) => {
                        let url = unsafe { SETTINGS.get_unchecked().remote_target.pages_url_iframe() };
                        url + &Route::Kids(KidsRoute::StudentCode(Some(student_code))).to_string()
                    },
                }
            }))
            .property_signal("code", state.student_code.signal_cloned().map(|student_code| {
                match student_code {
                    None => String::new(),
                    Some(student_code) => student_code,
                }
            }))
            .property_signal("secondsToExpire", state.student_code.signal_cloned().map(|student_code| {
                student_code.map(|_| JIG_PLAYER_SESSION_VALID_DURATION_SECS)
            }))
            .children(&mut [
                html!("share-jig-gen-code-button", {
                    .property("slot", "gen-code-button")
                    .property_signal("disabled", state.student_code.signal_ref(|x| x.is_some()))
                    .event(clone!(state => move |_: events::Click| {
                        state.generate_student_code();
                    }))
                }),
                html!("button-empty", {
                    .property("slot", "close")
                    .text("×")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text("< ")
                    .text(STR_BACK)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareMain));
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "copy-url")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text_signal(state.copied_student_url.signal().map(|copied| {
                        if copied { STR_STUDENTS_COPIED_URL_LABEL } else { STR_STUDENTS_COPY_URL_LABEL }
                    }))
                    .property_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
                    .event(clone!(state => move |_: events::Click| {
                        if let Some(student_code) = &*state.student_code.lock_ref() {
                            let url = unsafe { SETTINGS.get_unchecked().remote_target.pages_url_iframe() };
                            let url = url + &Route::Kids(KidsRoute::StudentCode(Some(student_code.clone()))).to_string();
                            clipboard::write_text(&url);
                            ShareAsset::set_copied_mutable(state.copied_student_url.clone());
                        };
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "copy-code")
                    .property("kind", "text")
                    .property("color", "blue")
                    .property_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
                    .text_signal(state.copied_student_code.signal().map(|copied| {
                        if copied { STR_STUDENTS_COPIED_CODE_LABEL } else { STR_STUDENTS_COPY_CODE_LABEL }
                    }))
                    .event(clone!(state => move|_: events::Click| {
                        let student_code = state.student_code.get_cloned().unwrap_ji();
                        clipboard::write_text(&student_code);
                        ShareAsset::set_copied_mutable(state.copied_student_code.clone());
                    }))
                }),
            ])
        })
    }

    fn render_share_embed(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(self);
        html!("share-jig-embed", {
            .property("slot", "overlay")
            .property("value", state.embed_code())
            .children(&mut [
                html!("button-empty", {
                    .property("slot", "close")
                    .text("×")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .property("slot", "back")
                    .property("kind", "text")
                    .text("< ")
                    .text(STR_BACK)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareMain));
                    }))
                }),
                html!("div", {
                    .property("slot", "copy")
                    .child(html!("button-rect", {
                        .property("kind", "text")
                        .text_signal(state.copied_embed.signal().map(|copied| {
                            if copied { STR_EMBED_COPIED_CODE_LABEL } else { STR_EMBED_COPY_CODE_LABEL }
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            clipboard::write_text(&state.embed_code());
                            ShareAsset::set_copied_mutable(state.copied_embed.clone());
                        }))
                    }))
                    .event_with_options(
                        &EventOptions::bubbles(),
                        |evt: events::Click| {
                            // stop close event from propagating to the anchored-overlay
                            evt.stop_propagation();
                        }
                    )
                })
            ])
        })
    }
}
