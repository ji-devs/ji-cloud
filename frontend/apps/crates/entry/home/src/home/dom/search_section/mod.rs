use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

use super::super::{
    actions::{fetch_data, search},
    state::Home,
};

pub fn render(state: Rc<Home>, auto_search: bool) -> Dom {
    fetch_data(state.clone(), auto_search);

    html!("home-search-section", {
        .prop_signal("mode", state.mode.signal_cloned().map(|mode| mode.to_string()))
        .prop_signal("resultsCount", state.total_assets_count.signal().map(|x| x as f64))
        // .child(html!("home-search-section-help", {
        //     .prop("slot", "help")
        // }))
        .child(state.search_bar.render_bar(Rc::new(clone!(state => move || {
            search(&state)
        }))))
    })
}
