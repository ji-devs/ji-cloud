use dominator::{html, Dom};
use shared::domain::module::body::_groups::cards::get_longest_card_text_length;

use super::{
    card::dom::{render_bottom, render_drag, render_top},
    state::*,
};
use std::rc::Rc;

use futures_signals::signal::SignalExt;

pub fn render(state: Rc<Game>) -> Dom {
    html!("matching-main", {
        .prop("slot", "main")
        .children_signal_vec(
            state.current.signal_cloned()
                .map(|current| {

                    let mut children:Vec<Dom> = Vec::new();

                    if let Some(current) = current {
                        let top_text_len = get_longest_card_text_length(current.top.iter().map(|top_choice| &top_choice.card));
                        let bottom_text_len = get_longest_card_text_length(current.bottom.iter().map(|top_choice| &top_choice.card));

                        for top in current.top.iter() {
                            children.push(render_top(top.clone(), top_text_len, bottom_text_len));
                        }

                        for bottom in current.bottom.iter() {
                            children.push(render_bottom(bottom.clone(), bottom_text_len));
                        }

                        children.push(
                            html!("empty-fragment", {
                                .prop("slot", "drag")
                                .style("position", "absolute")
                                .child_signal(current.drag.signal_cloned().map(move |drag| {
                                    drag.map(|drag| render_drag(drag, bottom_text_len))
                                }))
                            })
                        );
                    }

                    children
                })
                .to_signal_vec()
        )
    })
}
