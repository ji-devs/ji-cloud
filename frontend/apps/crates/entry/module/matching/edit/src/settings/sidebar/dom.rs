use dominator::{html, Dom, clone, with_node};
use std::rc::Rc;
use crate::{
    state::*,
    settings::state::*
};
use super::state::*;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::module::_groups::cards::lookup::Side;

pub fn render(state: Rc<SidebarSettings>) -> Dom {
    html!("card-quiz-settings", {
        .style("display", "flex")
        .style("flex-direction", "column")
        .child(render_top_side(state.clone()))
        .child(render_n_choices(state.clone()))
    })
}

fn render_top_side(state: Rc<SidebarSettings>) -> Dom {
    html!("button", {
        .text("Swap side")
        .event(clone!(state => move |evt:events::Click| {
            state.settings().top_side.replace_with(|side| {
                side.negate()
            });
        }))
    })
}

fn render_n_choices(state: Rc<SidebarSettings>) -> Dom {
    html!("label", {
        .text("Number of choices:")
        .child(
            html!("input" => web_sys::HtmlInputElement,{
                .property("type", "number")
                .property_signal("value", state.settings().n_choices.signal().map(|x| {
                    format!("{}", x)
                }))
                .with_node!(elem => {
                    .event(clone!(state => move |evt:events::Change| {
                        let value = elem.value();
                        if let Ok(value) = value.parse::<usize>() {
                            if value > 0 {
                                state.settings().n_choices.set_neq(value);
                            }
                        }
                    }))
                })
            })
        )
    })
}
