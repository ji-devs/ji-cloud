use futures_signals::signal::Mutable;

pub struct State {
    pub is_dragging: Mutable<bool>,
}

impl State {
    pub fn new() -> Self {
        Self {
            is_dragging: Mutable::new(false),
        }
    }
}
