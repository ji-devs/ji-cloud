use super::state::*;
use components::audio::input::AudioInput;
use dominator::{html, Dom};
use std::rc::Rc;

pub fn render(state: Rc<Step3>) -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .child({
            AudioInput::render(
                Rc::clone(&state.audio),
                None
            )
        })
    })
}
