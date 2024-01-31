use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use std::rc::Rc;

use components::{
    page_footer,
    page_header::{PageHeader, PageHeaderConfig, PageLinks},
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};

use super::state::{Home, HomePageMode};

mod home_sections;
mod iframe;
mod search_section;
use iframe::Iframe;

impl Home {
    pub fn render(self: Rc<Self>, auto_search: bool) -> Dom {
        let state = self;
        html!("home-full", {
            .child_signal(state.mode.signal_ref(|mode| {
                // Some(page_header::dom::render(Rc::new(page_header::state::PageHeader::new()), None, Some(PageLinks::from(mode)), true))
                Some(PageHeader::new(PageHeaderConfig {
                    active_page: Some(PageLinks::from(mode)),
                    ..Default::default()
                }).render())
            }))
            .children(&mut [
                search_section::render(state.clone(), auto_search),
                html!("empty-fragment", {
                    .child_signal(state.mode.signal_cloned().map(move |mode| {
                        match mode {
                            HomePageMode::Home => {
                                // Some(home_sections::render(state.clone()))
                                Some(Iframe::new().render())
                            },
                            HomePageMode::Search(search_results) => {
                                Some(search_results.render())
                            },
                        }
                    }))
                }),
                page_footer::dom::render(None),
            ])
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move|play_asset| {
                play_asset.map(|asset_id| {
                    let close = clone!(state => move || {
                        state.play_asset.set(None);
                    });
                    PlayerPopup::new_default_player_options(
                        asset_id,
                        PreviewPopupCallbacks::new(close)
                    ).render(None)
                })
            })))
        })
    }
}
