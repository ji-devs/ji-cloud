use super::state::*;
use crate::edit::sidebar::spot::pro_dev::actions::edit;
use crate::edit::sidebar::{
    pro_dev::actions as pro_dev_actions,
    spot::actions::{self, MoveTarget},
    state::ProDevSpot,
    state::Sidebar as SidebarState,
};
use dominator::{clone, html, Dom, EventOptions};
use shared::domain::module::ModuleId;
use shared::domain::pro_dev::unit::ProDevUnit;
use std::rc::Rc;
use utils::{
    events,
    routes::{AssetEditRoute, AssetRoute, ProDevEditRoute, Route},
};

impl ProDevMenu {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("menu-kebab", {
            .prop("slot", "menu")
            .child(html!("pro_dev-edit-sidebar-module-menu", {
                .children(state.menu_units())
            }))
            .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
                e.stop_propagation();
            })
            .after_inserted(move |elem| {
                *state.menu_ref.borrow_mut() = Some(elem);
            })
        })
    }

    fn menu_units(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let unit = state.spot_state.spot.item.unwrap_pro_dev();
        state.menu_units_pro_dev(unit)
    }

    fn menu_units_pro_dev(self: &Rc<Self>, unit: &Option<Rc<ProDevSpot>>) -> Vec<Dom> {
        let state = self;
        match unit {
            Some(module) => match &**module {
                ProDevSpot::Cover(cover) => {
                    vec![state.cover_edit(cover.id)]
                }
                ProDevSpot::Unit(pro_dev_unit) => {
                    vec![
                        state.unit_edit(),
                        state.unit_move_up(),
                        state.unit_move_down(),
                        state.unit_duplicate(
                            &state.spot_state.sidebar.clone(),
                            pro_dev_unit.clone(),
                        ),
                        state.unit_delete(),
                    ]
                }
            },
            None => {
                vec![]
            }
        }
    }

    fn cover_edit(self: &Rc<Self>, cover_id: ModuleId) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "edit")
            .event(clone!(state => move |_:events::Click| {
                let pro_dev_id = *state
                    .spot_state
                    .sidebar
                    .asset_edit_state
                    .asset_id
                    .unwrap_pro_dev();

                state
                    .spot_state
                    .sidebar
                    .asset_edit_state
                    .route
                    .set(AssetEditRoute::ProDev(pro_dev_id, ProDevEditRoute::Cover(cover_id)));

                state
                    .spot_state
                    .sidebar
                    .collapsed
                    .set(true);

                Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::ProDev(
                    pro_dev_id,
                    ProDevEditRoute::Cover(cover_id),
                ))));
            }))
        })
    }

    fn unit_edit(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "edit")
            .event(clone!(state => move |_:events::Click| {
                edit(state.spot_state.clone())
            }))
        })
    }

    fn unit_move_up(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "move-up")
            .event(clone!(state => move |_:events::Click| {
                state.close_menu();
                actions::move_index(state.spot_state.clone(), MoveTarget::Up);
            }))
        })
    }

    fn unit_move_down(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "move-down")
            .event(clone!(state => move |_:events::Click| {
                state.close_menu();
                actions::move_index(state.spot_state.clone(), MoveTarget::Down);
            }))
        })
    }

    fn unit_delete(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "delete")
            .event(clone!(state => move |_:events::Click| {
                state.spot_state.confirm_delete.set_neq(true);
                state.close_menu();
            }))
        })
    }

    fn unit_duplicate(self: &Rc<Self>, sidebar_state: &Rc<SidebarState>, unit: ProDevUnit) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "duplicate")
            .event(clone!(state, sidebar_state => move |_:events::Click| {
                state.close_menu();
                pro_dev_actions::duplicate_unit(sidebar_state.clone(), &unit);
            }))
        })
    }
}
