use std::rc::Rc;

use dominator::{html, Dom, clone, with_node};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::jig::JigId;
use utils::events;
use uuid::Uuid;
use web_sys::HtmlInputElement;

use super::state::JigSelection;


impl JigSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_course();
        html!("div", {
            .property("slot", "main")
            .children(&mut [
                html!("input" => HtmlInputElement, {
                    .style("width", "400px")
                    .with_node!(elem => {
                        .event(clone!(state => move|_: events::Input| {
                            let value = elem.value();
                            *state.input.borrow_mut() = value;
                        }))
                    })
                })
            ])
            .children_signal_vec(state.jigs.signal_vec_cloned().map(|jig| {
                html!("p", {
                    .text(&jig.0.to_string())
                })
            }))
            .child(html!("button", {
                .text("Button")
                .event(clone!(state => move|_: events::Click| {
                    let input = state.input.borrow();
                    let uuid = Uuid::parse_str(&input).unwrap();
                    let jig_id = JigId(uuid);
                    state.jigs.lock_mut().push(jig_id);
                    state.save_course();
                }))
            }))
        })
    }
}
