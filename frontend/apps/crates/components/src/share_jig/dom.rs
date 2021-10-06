use std::rc::Rc;

use dominator::{Dom, clone, html, with_node};
use futures_signals::signal::{Signal, SignalExt};
use shared::config::JIG_PLAYER_SESSION_VALID_DURATION_SECS;
use utils::{clipboard, events, routes::{KidsRoute, Route}, unwrap::UnwrapJiExt};

use crate::{animation::fade::{Fade, FadeKind}, tooltip::{
    state::{MoveStrategy, Anchor, ContentAnchor, State as TooltipState, TooltipBubble, TooltipData, TooltipTarget},
    dom::render as TooltipDom
}};

use super::{
    state::{ActivePopup, State},
    actions,
};

const STR_BACK: &'static str = "Back";
const STR_COPIED: &'static str = "Copied to the clipboard";
const STR_COPY_CODE: &'static str = "Copy Code";
const JIGZI_BASE_URL: &'static str = "https://jigzi.org";

pub fn render(state: Rc<State>, anchor: Dom, slot: Option<&str>) -> Dom {
    html!("anchored-overlay", {
        .apply_if(slot.is_some(), |dom| {
            dom.property("slot", slot.unwrap())
        })
        .property("positionY", "bottom-out")
        .property("positionX", "left-in")
        .property("styled", true)
        .property("slot", "actions")
        .property_signal("open", share_open_signal(Rc::clone(&state)))
        .event(clone!(state => move |_: events::Close| {
            state.active_popup.set(None);
        }))
        .child(html!("empty-fragment", {
            .property("slot", "anchor")
            .event(clone!(state => move |_: events::Click| {
                let new_value = match &*state.active_popup.lock_ref() {
                    Some(_) => None,
                    _ => Some(ActivePopup::ShareMain),
                };
                state.active_popup.set(new_value);
            }))
            .child(anchor)
        }))
        .child_signal(state.active_popup.signal_cloned().map(clone!(state => move|active_popup| {
            match active_popup {
                Some(ActivePopup::ShareMain) => {
                    Some(render_share_main(Rc::clone(&state)))
                },
                Some(ActivePopup::ShareStudents) => {
                    Some(render_share_students(Rc::clone(&state)))
                },
                Some(ActivePopup::ShareEmbed) => {
                    Some(render_share_embed(Rc::clone(&state)))
                },
                _ => None,
            }
        })))
    })
}

fn share_open_signal(state: Rc<State>) -> impl Signal<Item = bool> {
    state
        .active_popup
        .signal_cloned()
        .map(|active_popup| match active_popup {
            Some(_) => true,
            _ => false,
        })
}

fn render_share_main(state: Rc<State>) -> Dom {
    html!("share-jig-main", {
        .property("slot", "overlay")
        .children(&mut [
            html!("button-empty", {
                .property("slot", "close")
                .text("×")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(None);
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "students")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareStudents));
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "embed")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(Some(ActivePopup::ShareEmbed));
                }))
            }),
            html!("share-jig-option", {
                .property("kind", "copy")
                .event(|_: events::Click| {
                    clipboard::write_text("???");
                })
            }),
        ])
    })
}

fn render_share_students(state: Rc<State>) -> Dom {
    html!("share-jig-students", {
        .property("slot", "overlay")
        .property_signal("url", state.student_code.signal_cloned().map(|student_code| {
            match student_code {
                None => String::new(),
                Some(student_code) => {
                    let url = String::from(JIGZI_BASE_URL);
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
                    actions::generate_student_code(Rc::clone(&state));
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
                .text("Copy URL")
                .property_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
                .event(clone!(state => move |_: events::Click| {
                    if let Some(student_code) = &*state.student_code.lock_ref() {
                        let url = String::from(JIGZI_BASE_URL);
                        let url = url + &Route::Kids(KidsRoute::StudentCode(Some(student_code.clone()))).to_string();
                        clipboard::write_text(&url);
                    };
                }))
            }),
            html!("button-rect", {
                .property("slot", "copy-code")
                .property("kind", "text")
                .property("color", "blue")
                .property_signal("disabled", state.student_code.signal_ref(|x| x.is_none()))
                .text(STR_COPY_CODE)
                .event(clone!(state => move|_: events::Click| {
                    let student_code = state.student_code.get_cloned().unwrap_ji();
                    clipboard::write_text(&student_code);
                }))
            }),
        ])
    })
}

fn render_share_embed(state: Rc<State>) -> Dom {
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
                .with_node!(elem => {
                    .property("slot", "copy")
                    .child(html!("button-rect", {
                        .property("kind", "text")
                        .text("Copy code")
                        .event(clone!(state => move |_: events::Click| {
                            clipboard::write_text(&state.embed_code());
                            state.copied_embed.set(true);
                        }))
                    }))
                    .child_signal(state.copied_embed.signal().map(move |copied_embed| {
                        match copied_embed {
                            false => None,
                            true => {
                                let fade = Fade::new(
                                    FadeKind::Out,
                                    500.0,
                                    false,
                                    Some(4000.0),
                                    Some(clone!(state => move || {
                                        state.copied_embed.set(false);
                                    }))
                                );
        
                                Some(html!("div", {
                                    .apply(|dom| fade.render(dom))
                                    .child({
                                        let tooltip = TooltipData::Bubble(Rc::new(TooltipBubble {
                                            target_anchor: Anchor::MiddleRight,
                                            content_anchor: ContentAnchor::OppositeH,
                                            //slot: Some(String::from("copy")),
                                            body: String::from(STR_COPIED),
                                            max_width: None,
                                        }));
        
                                        let target = TooltipTarget::Element(elem.clone(), MoveStrategy::Track);
        
                                        TooltipDom(Rc::new(TooltipState::new(target, tooltip)))
                                    })
                                }))
                            }
                        }
                    }))
                })
                .event(|_evt: events::Click| {
                    // stop close event from propagating to the anchored-overlay
                    // TODO: this needs to be enabled once dominator allows it
                    // evt.stop_propagation();
                })
            })
        ])
    })
}
