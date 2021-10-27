use dominator::{html, Dom};

use super::{
    card::dom::{render_bottom, render_drag, render_top},
    state::*,
};
use std::rc::Rc;

use futures_signals::signal::SignalExt;

pub fn render(state: Rc<Game>) -> Dom {
    html!("matching-main", {
        .property("slot", "main")
        .children_signal_vec(
            state.current.signal_cloned()
                .map(|current| {

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
                })
                .to_signal_vec()
        )
    })
}
