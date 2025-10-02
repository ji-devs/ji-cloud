use std::rc::Rc;

use components::{
    page_header::PageHeader,
    player_popup::{PlayerPopup, PreviewPopupCallbacks},
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use super::state::Likes;

impl Likes {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("div", {
            .child(PageHeader::new(Default::default()).render())
            .child(html!("h1", {
                .style("height", "86px")
                .style("background-color", "var(--light-blue-6)")
                .style("margin", "0px")
                .style("margin-bottom", "20px")
                .style("display", "grid")
                .style("align-items", "center")
                .style("justify-content", "start")
                .style("padding-inline", "40px")
                .style("font-size", "29px")
                .style("font-weight", "900")
                .style("color", "var(--main-yellow)")
                .text("My likes")
            }))
            .child(state.playlists.render())
            .child(state.jigs.render())
            .child(state.resources.render())
            .child_signal(state.play_asset.signal_cloned().map(clone!(state => move |play_asset| {
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
