use components::overlay::handle::OverlayHandle;
use dominator::{Dom, EventOptions, clone, html, with_node, DomBuilder};
use web_sys::{HtmlElement, Node};

use super::super::menu::{dom as MenuDom};
use super::{actions, state::*};
use crate::edit::sidebar::state::State as SidebarState;
use components::module::_common::thumbnail::ModuleThumbnail;
use futures_signals::signal::SignalExt;
use shared::domain::jig::{LiteModule, ModuleKind};
use std::rc::Rc;
use std::str::FromStr;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
pub struct ModuleDom {}

impl ModuleDom {
    pub fn render(
        sidebar_state: Rc<SidebarState>,
        index: usize,
        drag_target_index: Option<usize>,
        total_len: usize,
        module: Rc<Option<LiteModule>>,
    ) -> Dom {
        let state = Rc::new(State::new(
            sidebar_state.clone(),
            index,
            total_len,
            module.clone(),
        ));

        let is_filler = Some(index) == drag_target_index;

        html!("empty-fragment", {
            .property("slot", if index == 0 { "cover-module" } else { "modules" })
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
                .property_signal("selected", state.sidebar.jig_edit_state.route.signal_ref(clone!(module => move |route| {
                    match (&*module, route) {
                        (Some(module), JigEditRoute::Module(module_id)) if module_id == &module.id => true,
                        _ => false,
                    }
                })))
                .property("lastBottomDecoration", index == total_len-1)
                // TODO:
                // .event(|_evt:events::MouseDown| {
                //     actions::mouse_down(state.clone(), evt.x(), evt.y());
                // })
                .event_with_options(
                    &EventOptions::bubbles(),
                    clone!(state => move |_evt:events::Click| {
                        let can_edit = match &*state.module {
                            Some(_) => {
                                if state.index == 0 {
                                    // If the cover is clicked, but the cover module hasn't yet
                                    // been dragged onto it, then it should navigate to the landing
                                    // screen.
                                    state.sidebar.first_cover_assigned.get()
                                } else {
                                    true
                                }
                            },
                            None => false,
                        };

                        if can_edit {
                            actions::edit(state.clone())
                        } else {
                            // Anything that cannot be edited should navigate the user to the
                            // landing screen.
                            state.sidebar.jig_edit_state.route.set_neq(JigEditRoute::Landing);
                        }

                    })
                )
                .child(html!("jig-edit-sidebar-module-window" => HtmlElement, {
                    .with_node!(elem => {
                        .property("slot", "window")
                        .property_signal("state", State::window_state_signal(Rc::clone(&state)))
                        .property("activeModuleKind", state.kind_str())
                        .property("coverOnly", state.index == 0)
                        .event_with_options(
                            &EventOptions::preventable(),
                            |evt:events::DragOver| {
                                if let Some(data_transfer) = evt.data_transfer() {
                                    if data_transfer.types().index_of(&JsValue::from_str("module_kind"), 0) != -1 {
                                        evt.prevent_default();
                                    }
                                }
                            }
                        )
                        .event_with_options(&EventOptions::preventable(), clone!(state => move |evt:events::Drop| {
                            evt.prevent_default(); // needed so that Firefox doesn't open the image
                            if let Some(data_transfer) = evt.data_transfer() {
                                if let Some(module_kind) = data_transfer.get_data("module_kind").ok() {
                                    let kind:ModuleKind = ModuleKind::from_str(&module_kind).unwrap_ji();
                                    actions::on_module_kind_drop(
                                        Rc::clone(&state),
                                        kind
                                    );
                                }
                            }
                        }))
                        .child_signal(state.sidebar.jig_edit_state.route.signal_ref(clone!(state, module => move |route| {
                            match (&*module, route) {
                                (Some(module), JigEditRoute::Module(module_id)) if module_id == &module.id => None,
                                (Some(module), _) => {
                                    Some(ModuleThumbnail::render_live(
                                        Rc::new(ModuleThumbnail {
                                            jig_id: state.sidebar.jig.id.clone(),
                                            module: Some(module.clone()),
                                            is_jig_fallback: false,
                                        }),
                                        Some("thumbnail")
                                    ))
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
                                                .text("You can only drag here the cover activity.")
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
                .apply_if(
                    module.is_some() || (index <= total_len - 2 && module.is_none()),
                    |dom| dom.child(MenuDom::render(&state))
                )
                .apply(Self::render_add_button(&state))
            }))
        })
    }

    pub fn render_add_button<A>(state: &Rc<State>) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> + '_
    where
        A: AsRef<Node>,
    {
        move |dom: DomBuilder<A>| {
            // If this module is anything other than a placeholder, the add button could be displayed.
            let current_module_should_add = state.module.is_some();
            let next_module_should_show_add = {
                match state.sidebar.modules.lock_ref().to_vec().get(state.index + 1) {
                    // If the next module is anything other than a placeholder, then this module can
                    // potentially display the the add button.
                    Some(module) => module.is_some(),
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
