use std::rc::Rc;

use dominator::{clone, html, with_node, Dom, EventOptions};
use futures_signals::signal::{Mutable, SignalExt};
use js_sys::encode_uri_component;
use shared::{config::JIG_PLAYER_SESSION_VALID_DURATION_SECS, domain::asset::Asset};
use utils::{
    clipboard, events, paywall,
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
const STR_MS_TEAMS: &str = "Share to Microsoft Teams";
const STR_STUDENTS_LABEL: &str = "Share with students";
const STR_EMBED_LABEL: &str = "Embed this ";
const STR_COPY_LABEL_1: &str = "Copy ";
const STR_COPY_LABEL_2: &str = " link";
const STR_COPIED_LABEL: &str = " link copied";

impl ShareAsset {
    pub fn render(self: Rc<Self>, anchor: Dom, slot: Option<&str>) -> Dom {
        let state = self;
        html!("empty-fragment" => HtmlElement, {
            .with_node!(elem => {
                .apply_if(slot.is_some(), |dom| {
                    dom.prop("slot", slot.unwrap_ji())
                })
                .event(clone!(state => move |_: events::Close| {
                    state.active_popup.set(None);
                }))
                .child(html!("empty-fragment", {
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
                            .prop("target", &elem)
                            .prop("contentAnchor", "oppositeH")
                            .prop("targetAnchor", "tr")
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

    fn can_play(self: &Rc<Self>) -> bool {
        let can_play = match &self.asset {
            Asset::Jig(jig) => paywall::can_play_jig(jig.admin_data.premium),
            Asset::Playlist(playlist) => paywall::can_play_playlist(playlist.admin_data.premium),
            Asset::Resource(resource) => paywall::can_play_resource(resource.admin_data.premium),
            Asset::Course(_) => paywall::can_play_course(),
        };
        if !can_play {
            paywall::dialog_limit(
                "
                Wanting to share our premium content?
                Upgrade now for UNLIMITED sharing options.
            ",
            );
        }
        can_play
    }

    fn render_share_main(self: &Rc<Self>) -> Dom {
        fn share_to(base: &str, url: &str) {
            if let Some(window) = window() {
                let share_url = format!("{}{}", base, encode_uri_component(url));
                let _ = window.open_with_url_and_target(&share_url, "_blank");
            }
        }

        // TODO: temporary until we have student-codes for playlists
        let temp_playlist_link_copied = Mutable::new(false);

        let state = self;
        html!("share-jig-main", {
            .prop("slot", "overlay")
            // TODO: temporary until we have student-codes for playlists
            .apply_if(state.asset.is_playlist(), |dom| {
                dom.child(html!("share-jig-option", {
                    .prop("kind", "students")
                    .text_signal(temp_playlist_link_copied.signal().map(clone!(state => move |copied| {
                        match copied {
                            false => STR_STUDENTS_LABEL.to_owned(),
                            true => format!("{}{STR_COPIED_LABEL}", state.asset_type_name()),
                        }
                    })))
                    .event(clone!(state => move |_: events::Click| {
                        if !state.can_play() {
                            return;
                        }
                        clipboard::write_text(&state.asset_link(true));
                        ShareAsset::set_copied_mutable(temp_playlist_link_copied.clone());
                    }))
                }))
            })
            .apply_if(state.asset.is_jig(), |dom| {
                dom.child(html!("share-jig-option", {
                    .prop("kind", "students")
                    .text(STR_STUDENTS_LABEL)
                    .event(clone!(state => move |_: events::Click| {
                        if !state.can_play() {
                            return;
                        }
                        state.active_popup.set(Some(ActivePopup::ShareStudents));
                    }))
                }))
            })
            .child(html!("share-jig-option", {
                .prop("kind", "google-classroom")
                .text(STR_CLASSROOM)
                .event(clone!(state => move |_: events::Click| {
                    if !state.can_play() {
                        return;
                    }
                    share_to("https://classroom.google.com/share?url=", &state.asset_link(true));
                }))
            }))
            .child(html!("share-jig-option", {
                .prop("kind", "ms-teams")
                .text(STR_MS_TEAMS)
                .event(clone!(state => move |_: events::Click| {
                    if !state.can_play() {
                        return;
                    }
                    share_to("https://teams.microsoft.com/share?href=", &state.asset_link(true));
                }))
            }))
            .apply_if(!state.asset.is_resource(), |dom| {
                dom.child(html!("share-jig-option", {
                    .prop("kind", "embed")
                    .text(&format!("{STR_EMBED_LABEL}{}", state.asset_type_name()))
                    .event(clone!(state => move |_: events::Click| {
                        if !state.can_play() {
                            return;
                        }
                        state.active_popup.set(Some(ActivePopup::ShareEmbed));
                    }))
                }))
            })
            .child(html!("share-jig-option", {
                .prop("kind", "copy")
                .text_signal(state.link_copied.signal().map(clone!(state => move |copied| {
                    match copied {
                        false => format!("{}{}{}", STR_COPY_LABEL_1, state.asset_type_name(), STR_COPY_LABEL_2),
                        true => format!("{}{STR_COPIED_LABEL}", state.asset_type_name()),
                    }
                })))
                .event(clone!(state => move|_: events::Click| {
                    clipboard::write_text(&state.asset_link(false));
                    ShareAsset::set_copied_mutable(state.link_copied.clone());
                }))
            }))
            .child(html!("fa-button", {
                .prop("slot", "close")
                .prop("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(None);
                }))
            }))
        })
    }

    fn render_share_students(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("share-jig-students", {
            .prop("slot", "overlay")
            .prop_signal("url", state.student_code.signal_cloned().map(|student_code| {
                match student_code {
                    None => String::new(),
                    Some(student_code) => {
                        let url = SETTINGS.get().unwrap_ji().remote_target.pages_url_iframe();
                        url + &Route::Kids(KidsRoute::StudentCode(Some(student_code))).to_string()
                    },
                }
            }))
            .prop_signal("code", state.student_code.signal_cloned().map(|student_code| {
                match student_code {
                    None => String::new(),
                    Some(student_code) => student_code,
                }
            }))
            .prop_signal("secondsToExpire", state.student_code.signal_cloned().map(|student_code| {
                student_code.map(|_| JIG_PLAYER_SESSION_VALID_DURATION_SECS)
            }))
            .children(&mut [
                html!("share-jig-gen-code-button", {
                    .prop("slot", "gen-code-button")
                    .prop_signal("disabled", state.student_code.signal_ref(|x| x.is_some()))
                    .event(clone!(state => move |_: events::Click| {
                        state.generate_student_code();
                    }))
                }),
                html!("button-empty", {
                    .prop("slot", "close")
                    .text("×")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "back")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text("< ")
                    .text(STR_BACK)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareMain));
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "copy-url")
                    .prop("color", "blue")
                    .prop("kind", "text")
                    .text_signal(state.copied_student_url.signal().map(|copied| {
                        if copied { STR_STUDENTS_COPIED_URL_LABEL } else { STR_STUDENTS_COPY_URL_LABEL }
                    }))
                    .prop_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
                    .event(clone!(state => move |_: events::Click| {
                        if let Some(student_code) = &*state.student_code.lock_ref() {
                            let url = SETTINGS.get().unwrap_ji().remote_target.pages_url_iframe();
                            let url = url + &Route::Kids(KidsRoute::StudentCode(Some(student_code.clone()))).to_string();
                            clipboard::write_text(&url);
                            ShareAsset::set_copied_mutable(state.copied_student_url.clone());
                        };
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "copy-code")
                    .prop("kind", "text")
                    .prop("color", "blue")
                    .prop_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
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
            .prop("slot", "overlay")
            .prop("assetTypeName", state.asset_type_name())
            .prop("value", state.embed_code())
            .children(&mut [
                html!("button-empty", {
                    .prop("slot", "close")
                    .text("×")
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(None);
                    }))
                }),
                html!("button-rect", {
                    .prop("slot", "back")
                    .prop("kind", "text")
                    .text("< ")
                    .text(STR_BACK)
                    .event(clone!(state => move |_: events::Click| {
                        state.active_popup.set(Some(ActivePopup::ShareMain));
                    }))
                }),
                html!("div", {
                    .prop("slot", "copy")
                    .child(html!("button-rect", {
                        .prop("kind", "text")
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
