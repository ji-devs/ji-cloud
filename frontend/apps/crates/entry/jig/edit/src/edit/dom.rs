use std::rc::Rc;

use super::{
    super::edit::publish::Publish,
    course::jig_selection::state::JigSelection,
    iframe::dom::IframeDom,
    post_publish::PostPublish,
    selection::dom::SelectionDom,
    sidebar::dom::SidebarDom,
    state::{AssetPlayerSettings, State},
};
use components::{
    overlay::handle::OverlayHandle,
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::{asset::AssetId, jig::JigFocus};
use utils::prelude::*;

const STR_YT_VIDEO_ID: &str = "x4FYtTpQAt0";

pub struct EditPage {}

impl EditPage {
    pub fn render(asset_id: AssetId, jig_focus: JigFocus, route: AssetEditRoute) -> Dom {
        let state = Rc::new(State::new(asset_id, jig_focus, route));

        html!("empty-fragment", {
            .child(html!("jig-edit-page", {
                /*
                 * this changes the url but does not preserve history
                 * commented out since it's misleading
                 * see the edit action for more comments
                 * we can navigate properly but then have full refresh
                 * might be worth popping/listening to location history
                 * but meh
                .future(module_id.signal().for_each(clone!(jig_id => move |module_id| {
                    let url:String = Route::Jig(JigRoute::Edit(jig_id, module_id)).into();

                    web_sys::window()
                        .unwrap_ji()
                        .history()
                        .unwrap_ji()
                        .push_state_with_url(&JsValue::NULL, "", Some(&url))
                        .unwrap_ji();
                    async {}
                })))
                */
                .apply_if(!state.jig_focus.is_resources(), |dom| {
                    dom.child(SidebarDom::render(asset_id, state.clone()))
                })
                .child_signal(state.route.signal_cloned().map(clone!(state, asset_id => move |route| {
                    match route {
                        AssetEditRoute::Jig(_jig_id, _jig_focus, jig_edit_route) => {
                            match jig_edit_route {
                                JigEditRoute::Landing => {
                                    if state.jig_focus.is_resources() {
                                        Some(Publish::render(Rc::clone(&state)))
                                    } else {
                                        Some(SelectionDom::render(state.clone()))
                                    }
                                },
                                JigEditRoute::Module(module_id) => {
                                    Some(IframeDom::render(*asset_id.unwrap_jig(), module_id))
                                },
                                JigEditRoute::Publish => {
                                    Some(Publish::render(Rc::clone(&state)))
                                }
                                JigEditRoute::PostPublish => {
                                    Some(PostPublish::new(
                                        *asset_id.unwrap_jig(),
                                        Rc::clone(&state)
                                    ).render())
                                }
                            }
                        },
                        AssetEditRoute::Course(course_id, course_edit_route) => {
                            match course_edit_route {
                                CourseEditRoute::Landing => {
                                    // Some(SelectionDom::render(state.clone()))
                                    // todo!()
                                    Some(JigSelection::new(course_id).render())
                                },
                                CourseEditRoute::Cover => {
                                    todo!()
                                },
                                CourseEditRoute::Publish => {
                                    Some(Publish::render(Rc::clone(&state)))
                                },
                                CourseEditRoute::PostPublish => {
                                    todo!()
                                },
                            }
                        },
                    }
                })))
                .child_signal(state.show_onboarding.signal_cloned().map(clone!(state => move |show| {
                    if show {
                        Some(html!("empty-fragment", {
                            .apply(OverlayHandle::lifecycle(clone!(state => move || {
                                html!("empty-fragment", {
                                    .child(html!("modal-video", {
                                        .property("videoId", STR_YT_VIDEO_ID)
                                        .event(clone!(state => move |_evt: events::Close| {
                                            state.set_permanently_closed();
                                        }))
                                    }))
                                })
                            })))
                        }))
                    } else {
                        None
                    }
                })))
            }))
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                play_jig.map(|settings| {
                    match settings {
                        AssetPlayerSettings::Jig(settings) => {
                            let close = clone!(state => move || {
                                state.play_jig.set(None);
                            });
                            PlayerPopup::new(
                                *asset_id.unwrap_jig(),
                                settings,
                                PreviewPopupCallbacks::new(close)
                            ).render(None)
                        },
                        AssetPlayerSettings::Course => {
                            todo!()
                        },
                    }
                })
            })))
        })
    }
}
