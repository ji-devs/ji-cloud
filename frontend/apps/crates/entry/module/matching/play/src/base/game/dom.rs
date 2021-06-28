use dominator::{html, Dom, DomBuilder, clone};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use std::rc::Rc;
use super::state::*;
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, render_card_mixin, CardOptions, render_empty_card, render_empty_card_mixin, EmptyCardOptions, EmptyKind, Size},
    edit::{
        config,
        state::*
    },
};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, ReadOnlyMutable}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card},
};
use rand::prelude::*;

use utils::prelude::*;

pub fn render(state: Rc<Game>) -> Dom {
    html!("matching-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.current.signal_cloned()
                .map(clone!(state => move |current| {
                    let mut children:Vec<Dom> = Vec::new();

                    let theme_id = state.base.theme_id.clone();
                    let mode = state.base.mode.clone();

                    let Current { top, bottom, side, phase } = current;

                    for item in top.iter() {

                        children.push(
                            html!("matching-column", {
                                .property("slot", "top")
                                .child({
                                    let mut options = CardOptions::new(&item.card, theme_id, mode, side, Size::Matching);
                                    options.flipped = true;
                                    render_card(options)
                                })
                                .child({
                                    let mut options = EmptyCardOptions::new(EmptyKind::Question, theme_id, Size::Matching);
                                    render_empty_card(options)
                                })
                            })
                        );
                    }

                    for item in bottom.iter() {
                        children.push({
                            let mut options = CardOptions::new(&item.card, theme_id, mode, side, Size::Matching);
                            options.flipped = true;
                            options.slot = Some("bottom"); 
                            render_card(options)
                        });
                    }

                    children
                }))
                .to_signal_vec()
        )
    })
}
