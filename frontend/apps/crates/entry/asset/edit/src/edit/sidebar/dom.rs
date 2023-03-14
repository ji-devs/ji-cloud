use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, with_node, Dom};
use dominator_helpers::events::Message;
use futures::future::ready;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

use super::{
    actions,
    dragging::{actions as drag_actions, dom::DraggingDom},
    header::dom::HeaderDom,
    jig::actions as jig_actions,
    spot::state::SpotState,
    state::*,
};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use std::rc::Rc;

use utils::{
    iframe::{IframeAction, ModuleToJigEditorMessage},
    prelude::*,
};

impl Sidebar {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("empty-fragment", {
            .prop("slot", "sidebar")
            .global_event(clone!(state => move |evt: Message| {
                match evt.try_serde_data::<IframeAction<ModuleToJigEditorMessage>>() {
                    Err(_e) => {

                    },
                    Ok(m) => {
                        jig_actions::on_iframe_message(Rc::clone(&state), m.data)
                    },
                };
            }))
            .child(html!("jig-edit-sidebar", {
                .future(clone!(state => async move {
                    state.asset_edit_state.route.signal_cloned().for_each(clone!(state => move |route| {
                        let should_collapse = !matches!(
                            route,
                            AssetEditRoute::Jig(_, JigEditRoute::Landing) | AssetEditRoute::Course(_, CourseEditRoute::Landing) | AssetEditRoute::ProDev(_, ProDevEditRoute::Landing) | AssetEditRoute::ProDev(_, ProDevEditRoute::Unit(_))
                        );
                        state.collapsed.set(should_collapse);
                        ready(())
                    })).await
                }))
                .prop_signal("collapsed", state.collapsed.signal())
                .prop_signal("isModulePage", state.asset_edit_state.route.signal_cloned().map(|route| {
                    // TODO: change?
                    matches!(
                        route,
                        AssetEditRoute::Jig(_, JigEditRoute::Landing) | AssetEditRoute::Course(_, CourseEditRoute::Landing) | AssetEditRoute::ProDev(_, ProDevEditRoute::Landing) | AssetEditRoute::ProDev(_, ProDevEditRoute::Unit(_))
                    )
                }))
                .prop_signal("loading", state.loader.is_loading())
                .child(html!("button-empty", {
                    .prop("slot", "side-head")
                    .child(html!("img-ui", {
                        .style("height", "60px")
                        .prop("path", "entry/jig/jiggling/yellow/face-small.png")
                    }))
                    .event(clone!(state => move |_ :events::Click| {
                        state.collapsed.set(false);
                    }))
                }))
                .child(HeaderDom::render(state.clone()))
                .child(html!("jig-edit-sidebar-publish", {
                    .prop("slot", "publish")
                    .prop("publish", state.asset_edit_state.asset.published_at().lock_ref().is_some())
                    .prop_signal("collapsed", state.collapsed.signal())
                    .prop_signal("selected", state.asset_edit_state.route.signal_cloned().map(|route| {
                        matches!(
                            route,
                            AssetEditRoute::Jig(_, JigEditRoute::Publish) | AssetEditRoute::Course(_, CourseEditRoute::Publish) | AssetEditRoute::ProDev(_, ProDevEditRoute::Publish)
                        )
                    }))
                    .event(clone!(state => move |_ :events::Click| {
                        if state.can_publish() {
                            actions::navigate_to_publish(state.clone());
                        } else {
                            actions::set_highlight_modules(&state, true);
                        }
                    }))
                    .child(html!("menu-kebab", {
                        .prop("slot", "menu")
                        .child(html!("menu-line", {
                            .prop("icon", "edit")
                        }))
                    }))
                    .with_node!(elem => {
                        .child_signal(state.highlight_modules.signal_cloned().map(clone!(state, elem => move |highlight| {
                            match highlight {
                                Some(ModuleHighlight::Publish) => {
                                    // Make sure that the publish window is visible to the teacher.
                                    elem.scroll_into_view_with_scroll_into_view_options(ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth));
                                    Some(html!("empty-fragment", {
                                        .apply(OverlayHandle::lifecycle(clone!(state, elem => move || {
                                            html!("overlay-tooltip-error", {
                                                .text("Your JIG has no content.")
                                                .prop("target", elem.clone())
                                                .prop("targetAnchor", "tr")
                                                .prop("contentAnchor", "oppositeH")
                                                .prop("closeable", true)
                                                .prop("strategy", "track")
                                                .style("width", "350px")
                                                .event(clone!(state => move |_:events::Close| {
                                                    state.highlight_modules.set_neq(None);
                                                }))
                                            })
                                        })))
                                    }))
                                },
                                _ => None,
                            }
                        })))
                    })
                }))
                .children_signal_vec(state.asset_edit_state.sidebar_spots
                    .signal_vec_cloned()
                    .enumerate()
                    .map_signal(clone!(state => move |(index, module)| {
                        map_ref! {
                            let len = state.asset_edit_state.sidebar_spots.signal_vec_cloned().len(),
                            let index = index.signal(),
                            let drag_target_index = state.drag_target_index.signal()
                                => move {
                                (
                                    index.unwrap_or_default(),
                                    *len,
                                    *drag_target_index,
                                    module.clone()
                                )
                            }
                        }
                    }))
                    .map(clone!(state => move |(index, len, drag_target_index, module)| {
                        SpotState::new(
                            state.clone(),
                            index,
                            drag_target_index,
                            len,
                            module
                        ).render()
                    }))
                )
                .global_event(clone!(state => move |evt:events::MouseUp| {
                    drag_actions::mouse_up(state.clone(), evt.x(), evt.y());
                }))
                .global_event(clone!(state => move |evt:events::MouseMove| {
                    drag_actions::mouse_move(state.clone(), evt.x(), evt.y());
                }))
            }))
            .child(DraggingDom::render(Rc::clone(&state)))
        })
    }
}

/*
*
       <jig-edit-sidebar ${slot && `slot="${slot}"`}>
       <jig-edit-sidebar-header slot="header"> </jig-edit-sidebar-header>
       ${mapToString(arrayIndex(nModules), index => {
           return Module({
               module: index === 0 ? "cover" : "memory",
               rawIndex: index,
               menuOpen: false,
               slot: index === 0 ? "cover-module" : "modules",
               selected: index === 1,
               makeDemoRoomAtTop: false,
               lastBottomDecoration: index === nModules-1
           });
       })}
       </jig-edit-sidebar>
       */
