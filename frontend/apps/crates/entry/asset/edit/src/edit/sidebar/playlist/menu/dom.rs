use super::state::*;
use crate::edit::sidebar::{
    spot::actions::{self, MoveTarget},
    state::PlaylistSpot,
};
use dominator::{clone, html, Dom, EventOptions};
use shared::domain::module::ModuleId;
use std::rc::Rc;
use utils::{
    events,
    routes::{AssetEditRoute, AssetRoute, PlaylistEditRoute, Route},
};

impl PlaylistMenu {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("menu-kebab", {
            .prop("slot", "menu")
            .child(html!("playlist-edit-sidebar-module-menu", {
                .children(state.menu_items())
            }))
            .event_with_options(&EventOptions::bubbles(), |e: events::Click| {
                e.stop_propagation();
            })
            .after_inserted(move |elem| {
                *state.menu_ref.borrow_mut() = Some(elem);
            })
        })
    }

    fn menu_items(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        let module = state.spot_state.spot.item.unwrap_playlist();
        state.menu_items_playlist(module)
    }

    fn menu_items_playlist(self: &Rc<Self>, module: &Option<Rc<PlaylistSpot>>) -> Vec<Dom> {
        let state = self;
        match module {
            Some(module) => match &**module {
                PlaylistSpot::Cover(cover) => {
                    vec![state.cover_edit(cover.id)]
                }
                PlaylistSpot::Item(_jig_id) => {
                    vec![
                        state.item_info(),
                        state.item_play(),
                        state.item_move_up(),
                        state.item_move_down(),
                        state.item_delete(),
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
                let playlist_id = *state
                    .spot_state
                    .sidebar
                    .asset_edit_state
                    .asset_id
                    .unwrap_playlist();

                state
                    .spot_state
                    .sidebar
                    .asset_edit_state
                    .route
                    .set(AssetEditRoute::Playlist(playlist_id, PlaylistEditRoute::Cover(cover_id)));

                state
                    .spot_state
                    .sidebar
                    .collapsed
                    .set(true);

                Route::push_state(Route::Asset(AssetRoute::Edit(AssetEditRoute::Playlist(
                    playlist_id,
                    PlaylistEditRoute::Cover(cover_id),
                ))));
            }))
        })
    }

    fn item_info(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "jig-info")
            .event(clone!(state => move |_:events::Click| {
                todo!("{:?}", state.spot_state.kind_str());
            }))
        })
    }

    fn item_play(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("menu-line", {
            .prop("slot", "lines")
            .prop("icon", "jig-play")
            .event(clone!(state => move |_:events::Click| {
                todo!("{:?}", state.spot_state.kind_str());
            }))
        })
    }

    fn item_move_up(self: &Rc<Self>) -> Dom {
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

    fn item_move_down(self: &Rc<Self>) -> Dom {
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

    fn item_delete(self: &Rc<Self>) -> Dom {
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
