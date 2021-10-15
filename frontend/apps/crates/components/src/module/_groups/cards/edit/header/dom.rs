use super::super::strings::STR_HEADER_ADD_PAIR;
use super::state::*;
use crate::buttons::{Button, ButtonStyle, ButtonStyleIcon};
use crate::module::{_common::edit::prelude::*, _groups::cards::edit::state::*};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;

impl<RawData: RawDataExt, E: ExtraExt> DomRenderable for Header<RawData, E> {
    fn render(state: Rc<Header<RawData, E>>) -> Dom {
        html!("empty-fragment", {
            .child_signal(state.show_add_pair_signal().map(clone!(state => move |show_add_pair| {
                if show_add_pair {
                    Some(Button::render(
                        Button::new_label(
                            ButtonStyle::Icon(ButtonStyleIcon::BluePlus),
                            String::from(STR_HEADER_ADD_PAIR),
                            clone!(state => move || {
                                state.add_pair();
                            })
                        ),
                        None
                    ))
                } else {
                    None
                }
            })))
        })
    }
}
