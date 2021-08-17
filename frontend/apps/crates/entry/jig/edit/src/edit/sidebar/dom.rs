use dominator::{clone, html, Dom};
use dominator_helpers::events::Message;

use super::{
    dragging::{actions as drag_actions, dom::DraggingDom},
    header::dom::HeaderDom,
    module::dom::ModuleDom,
    state::*,
    {actions, debug},
};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::{Jig, JigId};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

use utils::{iframe::{IframeAction, ModuleToJigEditorMessage}, prelude::*};

pub struct SidebarDom {}

impl SidebarDom {
    pub fn render(jig_id: JigId, route: Mutable<JigEditRoute>) -> Dom {
        let is_loading = Mutable::new(true);
        let jig = Rc::new(RefCell::new(None));

        html!("empty-fragment", {
            .property("slot", "sidebar")
            .future(clone!(is_loading, jig, jig_id => async move {
                if jig_id == JigId(Uuid::from_u128(0)) {
                    *jig.borrow_mut() = Some(debug::get_jig());
                } else {
                    actions::load_jig(jig_id, jig.clone()).await;
                }

                is_loading.set_neq(false);

            }))
            .child_signal(is_loading.signal().map(clone!(jig, route => move |loading| {
                if loading {
                    None
                } else {
                    Some(Self::render_loaded(jig.borrow_mut().take().unwrap_ji(), route.clone()))
                }
            })))
        })
    }

    fn render_loaded(jig: Jig, route: Mutable<JigEditRoute>) -> Dom {
        let state = Rc::new(State::new(jig, route));

        html!("empty-fragment", {
            .global_event(clone!(state => move |evt: Message| {
                match evt.try_serde_data::<IframeAction<ModuleToJigEditorMessage>>() {
                    Err(_e) => {
                        log::info!("{:?}", _e);
                    },
                    Ok(m) => {
                        actions::on_iframe_message(Rc::clone(&state), m.data)
                    },
                };
            }))
            .child(html!("jig-edit-sidebar", {
                .property_signal("collapsed", state.collapsed.signal())
                .property_signal("isModulePage", state.route.signal_cloned().map(|route| {
                    matches!(route, JigEditRoute::Landing)
                }))
                .property_signal("loading", state.loader.is_loading())
                .child(HeaderDom::render(state.clone()))
                .child(html!("jig-edit-sidebar-publish", {
                    .property("slot", "publish")
                    .property_signal("publish", state.publish_at.signal_cloned().map(|publish_at| {
                        publish_at.is_some()
                    }))
                    .property_signal("collapsed", state.collapsed.signal())
                    .property_signal("selected", state.route.signal_cloned().map(|route| {
                        matches!(route, JigEditRoute::Publish)
                    }))
                    .event(clone!(state => move |_ :events::Click| {
                        actions::navigate_to_publish(state.clone());
                    }))
                    .child(html!("menu-kebab", {
                        .property("slot", "menu")
                        .child(html!("menu-line", {
                            .property("icon", "edit")
                        }))
                    }))
                }))
                .children_signal_vec(state.modules
                    .signal_vec_cloned()
                    .enumerate()
                    .map_signal(clone!(state => move |(index, module)| {
                        map_ref! {
                            let len = state.modules.signal_vec_cloned().len(),
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
                        ModuleDom::render(state.clone(), index, drag_target_index, len, module)
                    }))
                )
                .global_event_preventable(clone!(state => move |evt:events::MouseUp| {
                    drag_actions::mouse_up(state.clone(), evt.x(), evt.y());
                }))
                .global_event_preventable(clone!(state => move |evt:events::MouseMove| {
                    drag_actions::mouse_move(state.clone(), evt.x(), evt.y());
                }))
            }))
            .child(DraggingDom::render(state.clone()))
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
