use dominator::{html, Dom, DomBuilder, clone};
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;
use std::rc::Rc;
use super::{
    state::*,
    card::{
        state::CardChoice,
        dom::{render_top, render_bottom, render_drag}
    }
};

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

                    if let Some(current) = current {

                        for top in current.top.iter() {
                            children.push(render_top(top.clone()));
                        }
                        for bottom in current.bottom.iter() {
                            children.push(render_bottom(bottom.clone()));
                        }

                        children.push(
                            html!("empty-fragment", {
                                .property("slot", "drag")
                                .style("position", "absolute")
                                .child_signal(current.drag.signal_cloned().map(|drag| {
                                    drag.map(render_drag)
                                }))
                            })
                        );
                    }

                    children
                }))
                .to_signal_vec()
        )
    })
}
