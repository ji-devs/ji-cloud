use std::rc::Rc;
use dominator::{Dom, clone, html};
use futures_signals::signal::Mutable;
use wasm_bindgen::{JsCast, prelude::Closure};
use crate::audio_input::state::State;

pub fn render(state: Rc<State>) -> Dom {
    let current_time = Rc::new(Mutable::new(0));
    events(state.clone(), current_time.clone());
    html!("progress-bar", {
        .property("slot", "main-content")
        .property("color", "green")
        .property_signal("progress", current_time.signal())
    })
}

fn events(state: Rc<State>, current_time: Rc<Mutable<i32>>) {
    let closure = Closure::wrap(Box::new(clone!(state, current_time => move |_: web_sys::Event| {
        let total = state.player.duration();
        let current = state.player.current_time();
        let time = time_to_percent(total, current);
        current_time.set(time as i32);
    })) as Box<dyn FnMut(_)>);
    state.player.add_event_listener_with_callback("timeupdate", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}


fn time_to_percent(total: f64, current: f64) -> f64 {
    (current / total) * 100_f64
}
