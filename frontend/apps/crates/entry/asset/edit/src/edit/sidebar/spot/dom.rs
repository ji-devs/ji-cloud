use components::overlay::handle::OverlayHandle;
use components::stickers::embed::types::ParseUrlExt;
use dominator::{clone, html, with_node, Dom, DomBuilder, EventOptions};
use futures_signals::map_ref;
use shared::domain::asset::DraftOrLive;
use shared::domain::course::unit::CourseUnitValue;
use web_sys::{HtmlElement, Node, ScrollBehavior, ScrollIntoViewOptions};

use super::super::jig::menu::JigMenu;
use super::super::playlist::menu::PlaylistMenu;
use super::super::spot::actions as spot_actions;
use super::course::actions as course_spot_actions;
use super::jig::actions as jig_spot_actions;
use super::{actions, state::*};
use crate::edit::sidebar::course::menu::CourseMenu;
use crate::edit::sidebar::state::{ModuleHighlight, PlaylistSpot, SidebarSpotItem};
use crate::edit::sidebar::CourseSpot;
use components::module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback};
use futures_signals::signal::{not, SignalExt};
use std::rc::Rc;
use utils::prelude::*;

const STR_DELETE_TITLE: &str = "Warning";
const STR_DELETE_CONTENT: &str = "Are you sure you want to delete this activity?";
const STR_DELETE_CONFIRM: &str = "Yes, delete";
const STR_DELETE_CANCEL: &str = "Don't delete";

impl SpotState {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        let is_filler = Some(state.index) == state.drag_target_index;

        let is_incomplete_signal = map_ref! {
            let is_complete = not(state.spot.is_incomplete.signal()),
            let highlight_modules = state.sidebar.highlight_modules.signal_cloned()
                => {
                    !is_complete && highlight_modules.is_some()
                }
        };

        html!("empty-fragment", {
            .prop("slot", if state.index == 0 { "cover-module" } else { "modules" })
            .child_signal(state.confirm_delete.signal().map(clone!(state => move |confirm_delete| {
                if confirm_delete {
                    Some(html!("modal-confirm", {
                        .prop("dangerous", true)
                        .prop("title", STR_DELETE_TITLE)
                        .prop("content", STR_DELETE_CONTENT)
                        .prop("cancel_text", STR_DELETE_CANCEL)
                        .prop("confirm_text", STR_DELETE_CONFIRM)
                        .prop("confirmIcon", "core/menus/delete-white.svg")
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
                .future(SpotState::drag_overlap_signal(state.clone()).for_each(clone!(state => move |overlap| {
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
                .prop("assetType", state.asset_type())
                .prop("module", state.kind_str())
                .prop("unit_name", state.unit_name())
                .prop("index", state.index as u32)
                .prop_signal("collapsed", state.sidebar.collapsed.signal())
                .prop_signal("selected", state.is_selected_signal())
                // TODO:
                // .event(|_evt:events::MouseDown| {
                //     actions::mouse_down(state.clone(), evt.x(), evt.y());
                // })
                .event_with_options(
                    &EventOptions::bubbles(),
                    clone!(state => move |_evt:events::Click| {
                        match &state.spot.item {
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
                            SidebarSpotItem::Playlist(item) => {
                                if let Some(item) = item {
                                    if let PlaylistSpot::Cover(cover) = &**item {
                                        state.sidebar.asset_edit_state.set_route_playlist(PlaylistEditRoute::Cover(cover.id));
                                        return;
                                    }
                                }
                                state.sidebar.asset_edit_state.set_route_playlist(PlaylistEditRoute::Landing);
                            },
                            SidebarSpotItem::Course(item) => {
                                match item {
                                    Some(item)=> {
                                        match &**item {
                                            CourseSpot::Cover(cover) => state.sidebar.asset_edit_state.set_route_course(CourseEditRoute::Cover(cover.id)),
                                            CourseSpot::Unit(_) => course_spot_actions::edit(state.clone()),
                                        }
                                    }
                                    None => state.sidebar.asset_edit_state.set_route_course(CourseEditRoute::Unit(None))
                                }
                            },
                        }
                    })
                )
                .child(html!("jig-edit-sidebar-module-window" => HtmlElement, {
                    .with_node!(elem => {
                        .prop("slot", "window")
                        .prop_signal("state", SpotState::window_state_signal(Rc::clone(&state)))
                        .prop_signal("incomplete", is_incomplete_signal)
                        .prop("activeModuleKind", state.kind_str())
                        .prop("coverOnly", state.index == 0)
                        .event(clone!(state => move |evt:events::CustomDrop| {
                            if let Some(data) = evt.detail().as_string() {
                                spot_actions::assign_to_empty_spot(&state, data);
                            }
                        }))
                        .child_signal(state.sidebar.asset_edit_state.route.signal_ref(clone!(state => move |route| {
                            match &state.spot.item {
                                SidebarSpotItem::Jig(module) => {
                                    match (&*module, route) {
                                        (Some(module), AssetEditRoute::Jig(_, JigEditRoute::Module(module_id))) if module_id == &module.id => None,
                                        (Some(module), _) => {
                                            Some(
                                                ModuleThumbnail::new(
                                                    state.sidebar.asset_edit_state.asset.id(),
                                                    Some((**module).clone()),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            )
                                        },
                                        _ => None,
                                    }
                                },
                                SidebarSpotItem::Playlist(playlist_spot) => {
                                    playlist_spot.as_ref().map(|playlist_spot| {
                                        match &**playlist_spot {
                                            PlaylistSpot::Cover(cover) => {
                                                ModuleThumbnail::new(
                                                    state.sidebar.asset_edit_state.asset.id(),
                                                    Some((*cover).clone()),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            },
                                            PlaylistSpot::Item(jig) => {
                                                ModuleThumbnail::new(
                                                    jig.id.into(),
                                                    jig.jig_data.modules.get(0).cloned(),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            },
                                        }
                                    })
                                }
                                SidebarSpotItem::Course(course_spot) => {
                                    course_spot.as_ref().map(|course_spot| {
                                        match &**course_spot {
                                            CourseSpot::Cover(cover) => {
                                                ModuleThumbnail::new(
                                                    state.sidebar.asset_edit_state.asset.id(),
                                                    Some((*cover).clone()),
                                                    ThumbnailFallback::Module,
                                                    DraftOrLive::Draft,
                                                ).render_live(Some("thumbnail"))
                                            },
                                            CourseSpot::Unit(unit) =>
                                            {
                                                match &unit.value {
                                                    CourseUnitValue::Video(youtube) => {
                                                        html!("div", {
                                                            .prop("slot", "unit")
                                                            .child(html!("video-youtube-thumbnail", {
                                                                .prop("videoId", youtube.url.get_id())
                                                                .style("width", "100%")
                                                                .style("height", "100%")
                                                            }))
                                                        })
                                                    },
                                                    CourseUnitValue::Link(_) => {
                                                        html!("div", {
                                                            .prop("slot", "unit")
                                                            .text(format!("Unit {}", state.index).as_str())
                                                            .child(html!("img-ui", {
                                                                .class("icon")
                                                                .prop("path", "entry/pro-dev/link-icon.svg")
                                                                .style("position", "absolute")
                                                                .style("height", "20px")
                                                                .style("width", "20px")
                                                                .style("top", "5px")
                                                                .style("left", "5px")
                                                            }))
                                                        })
                                                    },
                                                    CourseUnitValue::PdfId(_) => {
                                                        html!("div", {
                                                            .prop("slot", "unit")
                                                            .text(format!("Unit {}", state.index).as_str())
                                                            .child(html!("img-ui", {
                                                                .class("icon")
                                                                .prop("path", "entry/pro-dev/pdf-icon.svg")
                                                                .style("position", "absolute")
                                                                .style("height", "20px")
                                                                .style("width", "20px")
                                                                .style("top", "5px")
                                                                .style("left", "5px")
                                                            }))
                                                        })
                                                    },
                                                    CourseUnitValue::ImageId(image) => {
                                                        html!("div", {
                                                            .prop("slot", "unit")
                                                            .text(format!("Unit {}", state.index).as_str())
                                                            .child(html!("img-ji", {
                                                                .style("object-fit", "contain")
                                                                .prop("size", "full")
                                                                .prop("id", image.0.to_string())
                                                                .prop("lib", "user")
                                                            }))
                                                            .child(html!("img-ui", {
                                                                .class("icon")
                                                                .prop("path", "entry/pro-dev/image-icon.svg")
                                                                .style("position", "absolute")
                                                                .style("height", "20px")
                                                                .style("width", "20px")
                                                                .style("top", "5px")
                                                                .style("left", "5px")
                                                            }))
                                                        })
                                                    },
                                                    //TODO: Add Audio icon to thumbnail
                                                    _ => {
                                                        html!("div", {
                                                            .prop("slot", "unit")
                                                            .text(format!("Unit {}", state.index).as_str())
                                                        })
                                                    }
                                                }

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
                                        let scroll_into_view_options = ScrollIntoViewOptions::new();
                                        scroll_into_view_options.set_behavior(ScrollBehavior::Smooth);
                                        elem.scroll_into_view_with_scroll_into_view_options(&scroll_into_view_options);
                                        Some(html!("empty-fragment", {
                                            .apply(OverlayHandle::lifecycle(clone!(state, elem => move || {
                                                html!("overlay-tooltip-error", {
                                                    .text("This part of your JIG needs attention. Add content or delete.")
                                                    .prop("target", elem.clone())
                                                    .prop("targetAnchor", "tr")
                                                    .prop("contentAnchor", "oppositeH")
                                                    .prop("marginX", 75i32)
                                                    .prop("closeable", true)
                                                    .prop("strategy", "track")
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
                                Some(ModuleHighlight::Unit(idx)) => {
                                    if idx == state.index {
                                        let scroll_into_view_options = ScrollIntoViewOptions::new();
                                        scroll_into_view_options.set_behavior(ScrollBehavior::Smooth);
                                        elem.scroll_into_view_with_scroll_into_view_options(&scroll_into_view_options);
                                        Some(html!("empty-fragment", {
                                            // Populate existing unit with it's data.
                                        }))
                                    } else {
                                        None
                                    }
                                },
                                _ =>
                                {
                                    None
                                },
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
                                                .prop("target", elem.clone())
                                                .prop("targetAnchor", "mr")
                                                .prop("contentAnchor", "oppositeH")
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
                    //     Asset::Playlist(_) => todo!(),
                    // }

                    match state.spot.item {
                        SidebarSpotItem::Jig(_) => dom.child(JigMenu::new(&state).render()),
                        SidebarSpotItem::Playlist(_) => dom.child(PlaylistMenu::new(&state).render()),
                        SidebarSpotItem::Course(_) => dom.child(CourseMenu::new(&state).render()),
                    }
                }))
                .apply(Self::render_add_button(&state))
            }))
        })
    }

    pub fn render_add_button<A>(
        state: &Rc<SpotState>,
    ) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> + '_
    where
        A: AsRef<Node>,
    {
        move |dom: DomBuilder<A>| {
            // If this module is anything other than a placeholder, the add button could be displayed.
            let current_module_should_add = state.spot.item.is_some();
            let next_module_should_show_add = {
                match state
                    .sidebar
                    .asset_edit_state
                    .sidebar_spots
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
                    .prop("icon", "gears")
                    .prop("slot", "add")
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
