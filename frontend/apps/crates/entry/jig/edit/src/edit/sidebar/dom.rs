use dominator::{html, clone, Dom};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::{Jig, JigId, ModuleId};
use super::{
    {actions, debug},
    header::dom::HeaderDom,
    module::dom::ModuleDom,
    dragging::{
        dom::DraggingDom,
        actions as drag_actions
    },
    state::*,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use utils::events;

pub struct SidebarDom {
}

impl SidebarDom {
    pub fn render(jig_id: JigId, module_id: Mutable<Option<ModuleId>>) -> Dom {
        let loader = AsyncLoader::new();
        let jig = Rc::new(RefCell::new(None));

        loader.load(clone!(jig => async move {
            if jig_id == JigId(Uuid::from_u128(0)) {
                *jig.borrow_mut() = Some(debug::get_jig());
            } else {
                actions::load_jig(jig_id, jig.clone());
            }
        }));

        Dom::with_state(loader, clone!(jig => move |loader| {
            html!("empty-fragment", {
                .property("slot", "sidebar")
                .child_signal(loader.is_loading().map(clone!(jig => move |loading| {
                    if loading {
                        None
                    } else {
                        Some(Self::render_loaded(jig.borrow_mut().take().unwrap_throw()))
                    }
                })))
            })
        }))

    }

    fn render_loaded(jig: Jig) -> Dom {
        let state = Rc::new(State::new(jig));


        html!("empty-fragment", {
            .child(html!("jig-edit-sidebar", {
                .child(HeaderDom::render(state.clone()))
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
