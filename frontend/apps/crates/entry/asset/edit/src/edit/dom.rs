use std::rc::Rc;

use super::{
    super::edit::publish::Publish, course::jig_selection::state::JigSelection,
    jig::module_selection::dom::SelectionDom, module_iframe::ModuleIframe, sidebar::Sidebar,
    state::AssetEditState,
};
use components::{
    overlay::handle::OverlayHandle,
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom};
use dominator_helpers::events::Message;
use futures_signals::signal::SignalExt;
use utils::{asset::AssetPlayerOptions, prelude::*};

const STR_YT_VIDEO_ID: &str = "x4FYtTpQAt0";

impl AssetEditState {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

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
                .apply_if(!state.asset_id.is_resource_id(), |dom| {
                    let state = Sidebar::new(Rc::clone(&state));
                    dom.child(state.render())
                })
                .child_signal(state.route.signal_cloned().map(clone!(state => move |route| {
                    match route {
                        AssetEditRoute::Jig(_jig_id, jig_edit_route) => {
                            match jig_edit_route {
                                JigEditRoute::Landing => {
                                    Some(SelectionDom::render(state.clone()))
                                },
                                JigEditRoute::Module(module_id) => {
                                    Some(ModuleIframe::new(state.asset_id, module_id).render())
                                },
                                JigEditRoute::Publish => {
                                    Some(Publish::new(Rc::clone(&state)).render())
                                }
                            }
                        },
                        AssetEditRoute::Resource(_resource_id, resource_edit_route) => {
                            match resource_edit_route {
                                ResourceEditRoute::Landing => {
                                    Some(Publish::new(Rc::clone(&state)).render())
                                },
                                ResourceEditRoute::Cover(cover_id) => {
                                    Some(ModuleIframe::new(state.asset_id, cover_id).render())
                                },
                            }
                        },
                        AssetEditRoute::Course(course_id, course_edit_route) => {
                            match course_edit_route {
                                CourseEditRoute::Landing => {
                                    Some(JigSelection::new(course_id, &state).render())
                                },
                                CourseEditRoute::Cover(cover_id) => {
                                    Some(ModuleIframe::new(state.asset_id, cover_id).render())
                                },
                                CourseEditRoute::Publish => {
                                    Some(Publish::new(Rc::clone(&state)).render())
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
                                        .prop("videoId", STR_YT_VIDEO_ID)
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
                .global_event(clone!(state => move |evt: Message| {
                    if let Ok(m) = evt.try_serde_data::<IframeAction<ModuleToAssetEditorMessage>>() {
                        state.on_iframe_message(m.data);
                    };
                }))
            }))
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                play_jig.map(|settings| {
                    match settings {
                        AssetPlayerOptions::Jig(settings) => {
                            let close = clone!(state => move || {
                                state.play_jig.set(None);
                            });
                            PlayerPopup::new(
                                state.asset_id,
                                None,
                                settings.into(),
                                PreviewPopupCallbacks::new(close)
                            ).render(None)
                        },
                        AssetPlayerOptions::Course(settings) => {
                            let close = clone!(state => move || {
                                state.play_jig.set(None);
                            });
                            PlayerPopup::new(
                                state.asset_id,
                                None,
                                settings.into(),
                                PreviewPopupCallbacks::new(close)
                            ).render(None)
                        },
                    }
                })
            })))
        })
    }
}
