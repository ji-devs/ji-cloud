use super::state::*;
use crate::edit::sidebar::{
    spot::actions::{self, MoveTarget},
    state::ProDevSpot,
};
use dominator::{clone, html, Dom, EventOptions};
use shared::domain::module::ModuleId;
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
        let module = state.spot_state.spot.item.unwrap_pro_dev();
        state.menu_units_pro_dev(module)
    }

    fn menu_units_pro_dev(self: &Rc<Self>, module: &Option<Rc<ProDevSpot>>) -> Vec<Dom> {
        let state = self;
        match module {
            Some(module) => match &**module {
                ProDevSpot::Cover(cover) => {
                    vec![state.cover_edit(cover.id)]
                }
                ProDevSpot::Unit(_pro_dev_unit) => {
                    vec![
                        state.unit_info(),
                        state.unit_play(),
                        state.unit_move_up(),
                        state.unit_move_down(),
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

    fn unit_info(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "jig-info")
            .event(clone!(state => move |_:events::Click| {
                todo!("{:?}", state.spot_state.kind_str());
            }))
        })
    }

    fn unit_play(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "jig-play")
            .event(clone!(state => move |_:events::Click| {
                todo!("{:?}", state.spot_state.kind_str());
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
}
