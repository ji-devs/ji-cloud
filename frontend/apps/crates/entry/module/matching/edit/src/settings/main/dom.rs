use dominator::{html, Dom, clone};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use components::module::_groups::cards::{
    lookup::{self, Side},
    play::card::dom::{render_card, CardOptions, render_empty_card, EmptyCardOptions, EmptyKind, Size},
    edit::{
        config,
        state::*
    },
};
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, ReadOnlyMutable},
    signal_vec::{SignalVec, SignalVecExt}
};

use shared::domain::jig::module::body::{
    ThemeId,
    ModeExt,
    _groups::cards::{Mode, Step, Card}
};
use rand::prelude::*;

use utils::prelude::*;

pub fn render(state: Rc<MainSettings>) -> Dom {
    html!("matching-main", {
        .property("slot", "main")
        .children_signal_vec(render_top_choices(state.clone()))
        //.children_signal_vec(render_top_choices(state.clone(), state.top_choices_signal(), None))
        .children_signal_vec(render_bottom_choices(state.clone()))

    })
}

fn render_top_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {
    state.top_choices_signal()
        .to_signal_vec()
        .map_signal(clone!(state => move |choice| {
            let mode = state.base.mode.clone();
            //theme_id won't have actually changed here, but w/e
            state.base.theme_id.signal_cloned()
                .map(move |theme_id| {
                    html!("matching-column", {
                        .property("slot", "top")
                        .child({
                            let (card, side) = &choice;
                            let mut options = CardOptions::new(&card, theme_id, mode, *side, Size::Matching);
                            options.flipped = true;
                            render_card(options)
                        })
                        .child({
                            let mut options = EmptyCardOptions::new(EmptyKind::Question, theme_id, Size::Matching);
                            render_empty_card(options)
                        })
                    })
                })
        }))
}
fn render_bottom_choices(state: Rc<MainSettings>) -> impl SignalVec<Item = Dom> {
    state.bottom_choices_signal()
        .to_signal_vec()
        .map_signal(clone!(state => move |choice| {
            let mode = state.base.mode.clone();
            //theme_id won't have actually changed here, but w/e
            state.base.theme_id.signal_cloned()
                .map(move |theme_id| {
                    let (card, side) = &choice;
                    let mut options = CardOptions::new(&card, theme_id, mode, *side, Size::Matching);
                    options.flipped = true;
                    options.slot = Some("bottom"); 
                    render_card(options)
                })
        }))
}
/*
 *      <module-page-grid-resize>
        <matching-main slot="main" ${argsToAttrs(props)} ${style} >
          ${mapToString(arrayCount(nPairs), idx => {
            return renderTop(idx);
          })}
          ${mapToString(arrayCount(nPairs), idx => {
            return renderBottom(idx);
          })}
          ${renderFloating()}
        </matching-main>
      </module-page-grid-resize>
      */
