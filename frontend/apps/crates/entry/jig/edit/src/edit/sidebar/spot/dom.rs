use components::overlay::handle::OverlayHandle;
use dominator::{clone, html, with_node, Dom, DomBuilder, EventOptions};
use futures_signals::map_ref;
use shared::domain::asset::DraftOrLive;
use web_sys::{HtmlElement, Node, ScrollBehavior, ScrollIntoViewOptions};

use super::super::course::menu::dom as CourseMenuDom;
use super::super::jig::menu::dom as JigMenuDom;
use super::super::spot::actions as spot_actions;
use super::jig::actions as jig_spot_actions;
use super::{actions, state::*};
use crate::edit::sidebar::state::{
    CourseSpot, ModuleHighlight, SidebarSpot, SidebarSpotItem, State as SidebarState,
};
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use futures_signals::signal::{not, SignalExt};
use shared::domain::module::ModuleKind;
use std::rc::Rc;
use std::str::FromStr;
use utils::prelude::*;

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this activity?";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

pub struct ItemDom {}

impl ItemDom {
    pub fn render(
        sidebar_state: Rc<SidebarState>,
        index: usize,
        drag_target_index: Option<usize>,
        total_len: usize,
        module: Rc<SidebarSpot>,
    ) -> Dom {
        let state = Rc::new(State::new(
            sidebar_state.clone(),
            index,
            total_len,
            module.clone(),
        ));

        let is_filler = Some(index) == drag_target_index;

        let is_incomplete_signal = map_ref! {
            let is_complete = not(state.module.is_incomplete.signal()),
            let highlight_modules = sidebar_state.highlight_modules.signal_cloned()
                => {
                    !is_complete && highlight_modules.is_some()
                }
        };

        html!("empty-fragment", {
            .property("slot", if index == 0 { "cover-module" } else { "modules" })
            .child_signal(state.confirm_delete.signal().map(clone!(state => move |confirm_delete| {
                if confirm_delete {
                    Some(html!("modal-confirm", {
                        .property("dangerous", true)
                        .property("title", STR_DELETE_TITLE)
                        .property("content", STR_DELETE_CONTENT)
                        .property("cancel_text", STR_DELETE_CANCEL)
                        .property("confirm_text", STR_DELETE_CONFIRM)
                        .property("confirmIcon", "core/menus/delete-white.svg")
                        .event(clone!(state => move |_evt: events::CustomCancel| state.confirm_delete.set_neq(false)))
                        .event(clone!(state => move |_evt: events::CustomConfirm| {
                            state.confirm_delete.set_neq(false);
                            spot_actions::delete(state.clone());
                        }))
                    }))
                } else {
                    None
                }
            })))
            .child(html!("jig-edit-sidebar-filler", {
                .style("display", {
                    if is_filler { "block" } else {"none"}
                })
            }))
            .child(html!("jig-sidebar-module", {
                .future(State::drag_overlap_signal(state.clone()).for_each(clone!(state => move |overlap| {
                    if overlap {
                        state.sidebar.drag_target_index.set(Some(state.index));
                    }
                    // Doing this here instead of immediately on mousemove
                    // gives us a nice separation of concerns
                    // e.g. to throttle
                    // actions::update_index(state.clone(), pos.x, pos.y);
                    async {}
                })))
                .style("display", {
                    if is_filler { "none" } else {"block"}
                })
                .property("module", state.kind_str())
                .property("index", index as u32)
                .property_signal("collapsed", state.sidebar.collapsed.signal())
                .property_signal("selected", state.is_selected_signal())
                // TODO:
                // .event(|_evt:events::MouseDown| {
                //     actions::mouse_down(state.clone(), evt.x(), evt.y());
                // })
                .event_with_options(
                    &EventOptions::bubbles(),
                    clone!(state => move |_evt:events::Click| {
                        match &state.module.item {
                            SidebarSpotItem::Jig(module) => {
                                match module {
                                    Some(_) => {
                                        jig_spot_actions::edit(state.clone())
                                    },
                                    None => {
                                        state.sidebar.asset_edit_state.set_route_jig(JigEditRoute::Landing);
                                    },
                                }
                            },
                            SidebarSpotItem::Course(_course_spot) => {
                                todo!()
                            }
                        }
                    })
                )
                .child(html!("jig-edit-sidebar-module-window" => HtmlElement, {
                    .with_node!(elem => {
                        .property("slot", "window")
                        .property_signal("state", State::window_state_signal(Rc::clone(&state)))
                        .property_signal("incomplete", is_incomplete_signal)
                        .property("activeModuleKind", state.kind_str())
                        .property("coverOnly", state.index == 0)
                        .event(clone!(state => move |evt:events::CustomDrop| {
                            if let Some(detail) = evt.detail().as_string() {
                                if let Ok(kind) = ModuleKind::from_str(&detail) {
                                    actions::on_module_kind_drop(
                                        Rc::clone(&state),
                                        kind
                                    );
                                }
                            }
                        }))
                        .child_signal(state.sidebar.asset_edit_state.route.signal_ref(clone!(state, module => move |route| {
                            match &module.item {
                                SidebarSpotItem::Jig(module) => {
                                    match (&*module, route) {
                                        (Some(module), AssetEditRoute::Jig(_, JigEditRoute::Module(module_id))) if module_id == &module.id => None,
                                        (Some(module), _) => {
                                            Some(
                                                ModuleThumbnail::new(
                                                    state.sidebar.asset.id(),
                                                    Some((**module).clone()),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            )
                                        },
                                        _ => None,
                                    }
                                },
                                SidebarSpotItem::Course(course_spot) => {
                                    course_spot.as_ref().map(|course_spot| {
                                        match &**course_spot {
                                            CourseSpot::Cover(cover) => {
                                                ModuleThumbnail::new(
                                                    state.sidebar.asset.id(),
                                                    Some((*cover).clone()),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            },
                                            CourseSpot::Item(jig_id) => {
                                                ModuleThumbnail::new(
                                                    (*jig_id).into(),
                                                    None,
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            },
                                        }
                                    })
                                }
                            }
                        })))
                        .child_signal(state.sidebar.highlight_modules.signal_cloned().map(clone!(state, elem => move |highlight| {
                            match highlight {
                                Some(ModuleHighlight::Module(idx)) => {
                                    if idx == state.index {
                                        // Make sure that the module window is visible to the
                                        // teacher.
                                        elem.scroll_into_view_with_scroll_into_view_options(ScrollIntoViewOptions::new().behavior(ScrollBehavior::Smooth));
                                        Some(html!("empty-fragment", {
                                            .apply(OverlayHandle::lifecycle(clone!(state, elem => move || {
                                                html!("overlay-tooltip-error", {
                                                    .text("This part of your JIG needs attention. Add content or delete.")
                                                    .property("target", elem.clone())
                                                    .property("targetAnchor", "tr")
                                                    .property("contentAnchor", "oppositeH")
                                                    .property("marginX", 75i32)
                                                    .property("closeable", true)
                                                    .property("strategy", "track")
                                                    .style("width", "350px")
                                                    .event(clone!(state => move |_:events::Close| {
                                                        state.sidebar.highlight_modules.set_neq(None);
                                                    }))
                                                })
                                            })))
                                        }))
                                    } else {
                                        None
                                    }
                                },
                                _ => None,
                            }
                        })))
                        .apply(OverlayHandle::lifecycle(clone!(state => move || {
                            html!("empty-fragment", {
                                .child_signal(state.tried_module_at_cover.signal().map(clone!(state, elem => move |tried_module_at_cover| {
                                    match tried_module_at_cover {
                                        false => None,
                                        true => {
                                            Some(html!("overlay-tooltip-error", {
                                                .text("Only the cover can be dragged here.")
                                                .property("target", elem.clone())
                                                .property("targetAnchor", "mr")
                                                .property("contentAnchor", "oppositeH")
                                                .style("width", "650px")
                                                .event(clone!(state => move |_:events::Close| {
                                                    state.tried_module_at_cover.set(false);
                                                }))
                                            }))
                                        },
                                    }
                                })))
                            })
                        })))
                    })
                }))
                .after_inserted(clone!(state => move |dom| {
                    *state.elem.borrow_mut() = Some(dom);
                    //*state.sidebar.drag_targets.borrow_mut().
                }))
                .after_removed(clone!(state => move |_dom| {
                    *state.elem.borrow_mut() = None;
                }))
                // Add the menu only if the current module is not a placeholder or if it is, it is
                // not the _last_ placeholder.
                // .apply_if(
                //     module.item.is_some() || (index <= total_len - 2 && module.item.is_none()), // TODO: total_len - 2 might overflow
                //     |dom| dom.child(MenuDom::render(&state))
                // )
                .apply(clone!(state => move|dom| {
                    // match state.module {
                    //     Asset::Jig(_) => {
                    //         if module.item.is_some() || (index <= total_len - 2 && module.item.is_none()) { // TODO: total_len - 2 might overflow
                    //             dom.child(MenuDom::render(&state))
                    //         } else {
                    //             dom
                    //         }
                    //     },
                    //     Asset::Course(_) => todo!(),
                    // }

                    match module.item {
                        SidebarSpotItem::Jig(_) => dom.child(JigMenuDom::render(&state)),
                        SidebarSpotItem::Course(_) => dom.child(CourseMenuDom::render(&state)),
                    }
                }))
                .apply(Self::render_add_button(&state))
            }))
        })
    }

    pub fn render_add_button<A>(
        state: &Rc<State>,
    ) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> + '_
    where
        A: AsRef<Node>,
    {
        move |dom: DomBuilder<A>| {
            // If this module is anything other than a placeholder, the add button could be displayed.
            let current_module_should_add = state.module.item.is_some();
            let next_module_should_show_add = {
                match state
                    .sidebar
                    .modules
                    .lock_ref()
                    .to_vec()
                    .get(state.index + 1)
                {
                    // If the next module is anything other than a placeholder, then this module can
                    // potentially display the the add button.
                    Some(module) => module.item.is_some(),
                    // If there is no next module, then this module can potentially display the add
                    // button.
                    None => true,
                }
            };

            let should_add = current_module_should_add && next_module_should_show_add;

            if should_add {
                dom.child(html!("button-icon", {
                    .property("icon", "gears")
                    .property("slot", "add")
                    .event(clone!(state => move |_evt:events::Click| {
                        actions::add_empty_module_after(state.clone())
                    }))
                }))
            } else {
                dom
            }
        }
    }
}
