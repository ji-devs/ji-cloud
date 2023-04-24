use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use shared::domain::pro_dev::ProDevResponse;
use shared::{
    api::endpoints::pro_dev,
    domain::{
        asset::DraftOrLive,
        pro_dev::{ProDevGetDraftPath, ProDevGetLivePath},
    },
};
use utils::{events, prelude::ApiEndpointExt};

use super::state::ProDevPlayer;

pub fn load_data(state: Rc<ProDevPlayer>) {
    state.loader.load(clone!(state => async move {
        load_pro_dev(Rc::clone(&state)).await;
    }));
}

async fn load_pro_dev(state: Rc<ProDevPlayer>) {
    state.loader.load(clone!(state => async move {
        let pro_dev = match state.player_options.draft_or_live {
            DraftOrLive::Live => {
                let pro_dev = {
                    pro_dev::GetLive::api_no_auth(ProDevGetLivePath(state.pro_dev_id), None).await
                };


                pro_dev
            },
            DraftOrLive::Draft => {
                let pro_dev = {
                    pro_dev::GetDraft::api_no_auth(ProDevGetDraftPath(state.pro_dev_id), None).await
                };

                pro_dev
            },
        };

        match pro_dev {
            Ok(pro_dev) => {
                // state.active_unit.set(Some(resp.pro_dev.units[0].clone()));
                if let Some(start_unit_id) = state.start_unit_id {
                    if let Some((index, _)) = pro_dev.pro_dev_data.units.iter().enumerate().find(|unit| {
                        unit.1.id == start_unit_id
                    }) {
                        state.active_unit.set_neq(Some(index));
                    };
                }
                state.pro_dev.set(Some(pro_dev));
            },
            Err(_) => {
                todo!();
            },
        }
    }));
}

pub fn page_forward_signal(
    state: Rc<ProDevPlayer>,
    pro_dev: &ProDevResponse,
) -> impl Signal<Item = bool> {
    state.current_page.signal().map(clone!(pro_dev => move |current_page| {
        let units_per_page = 10;
        let num_pages = (pro_dev.pro_dev_data.units.len() + units_per_page - 1) / units_per_page;
        let page = if let Some(page) = current_page {
            page + 1
        } else {
            // If the current page is not set, default to the first page
            0
        };
        // Calculate whether the current page is the last page
        let is_last_page = page >= (num_pages - 1);
        is_last_page // Return the bool signal
    }))
}

pub fn paginate(state: &Rc<ProDevPlayer>, pro_dev: &ProDevResponse) -> Dom {
    let units_per_page = 10;

    let current_page = state.current_page.get().unwrap_or(0);

    let start_index = current_page * units_per_page;

    let end_index = ((current_page + 1) * units_per_page).min(pro_dev.pro_dev_data.units.len());

    let units_to_display = &pro_dev.pro_dev_data.units[start_index..end_index];

    // Create buttons for each unit on the current page
    let unit_buttons =
        units_to_display
            .iter()
            .enumerate()
            .map(clone!(state => move |(index, _unit)| {
                html!("button", {
                    .text(&((current_page * units_per_page) + index + 1).to_string())
                    .event(clone!(state, index => move |_: events::Click| {
                        state.active_unit.set(Some(current_page * units_per_page + index));
                    }))
                })
            }));

    html!("div", {
        .children(
            unit_buttons
        )
    })
}
