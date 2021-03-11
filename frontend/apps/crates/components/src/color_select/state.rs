use std::rc::Rc;
use chrono::{DateTime, Utc};
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;

// system colors
// #00000000 #ffffffff #fff445ff #fac72dff #feae2aff #f34826ff #fb178dff #da0f63ff #f74ac8ff #9517acff #7a28fbff
// #414cb3ff #2d9bf0ff #22cdd4ff #18a789ff #8fd150ff #cfe741ff #bbccf8ff #dce9f5ff #e6e6e6ff #808080ff #1a1a1aff

pub type Color = String;

#[derive(Clone)]
pub struct UserColor {
    pub color: Color,
    pub time_created: DateTime<Utc>,
}

pub struct State {
    pub value: Mutable<Option<Color>>,
    pub system_colors: MutableVec<Rc<Color>>,
    pub user_colors: MutableVec<Rc<Option<UserColor>>>,
}
