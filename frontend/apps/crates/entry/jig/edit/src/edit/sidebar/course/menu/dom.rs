use super::state::*;
use crate::edit::sidebar::{
    spot::{
        actions::{self, MoveTarget},
        state::State as SpotState,
    },
    state::CourseSpot,
};
use dominator::{clone, html, Dom, EventOptions};
use std::rc::Rc;
use utils::events;

pub fn render(module_state: &Rc<SpotState>) -> Dom {
    let state = Rc::new(State::new());

    html!("menu-kebab", {
        .property("slot", "menu")
        .child(html!("course-edit-sidebar-module-menu", {
            .children(menu_items(&state, module_state))
        }))
        .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
            e.stop_propagation();
        })
        .after_inserted(move |elem| {
            *state.menu_ref.borrow_mut() = Some(elem);
        })
    })
}

fn menu_items(state: &Rc<State>, module_state: &Rc<SpotState>) -> Vec<Dom> {
    let module = module_state.module.item.unwrap_course();
    menu_items_course(state, module_state, module)
}

fn menu_items_course(
    state: &Rc<State>,
    module_state: &Rc<SpotState>,
    module: &Option<Rc<CourseSpot>>,
) -> Vec<Dom> {
    match module {
        Some(module) => match &**module {
            CourseSpot::Cover(_cover) => {
                vec![cover_edit(state, module_state)]
            }
            CourseSpot::Item(_jig_id) => {
                vec![
                    item_info(state, &module_state),
                    item_play(state, &module_state),
                    item_move_up(state, &module_state),
                    item_move_down(state, &module_state),
                    item_delete(state, &module_state),
                ]
            }
        },
        None => {
            vec![]
        }
    }
}

fn cover_edit(_: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "edit")
        .event(clone!(module => move |_:events::Click| {
            todo!("{:?}", module.kind_str());
        }))
    })
}

fn item_info(_: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "edit")
        .event(clone!(module => move |_:events::Click| {
            todo!("{:?}", module.kind_str());
        }))
    })
}

fn item_play(_: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "edit")
        .event(clone!(module => move |_:events::Click| {
            todo!("{:?}", module.kind_str());
        }))
    })
}

fn item_move_up(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-up")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Up);
        }))
    })
}

fn item_move_down(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "move-down")
        .event(clone!(state, module => move |_:events::Click| {
            state.close_menu();
            actions::move_index(module.clone(), MoveTarget::Down);
        }))
    })
}

fn item_delete(state: &Rc<State>, module: &Rc<SpotState>) -> Dom {
    html!("menu-line", {
        .property("slot", "lines")
        .property("icon", "delete")
        .event(clone!(state, module => move |_:events::Click| {
            module.confirm_delete.set_neq(true);
            state.close_menu();
        }))
    })
}
