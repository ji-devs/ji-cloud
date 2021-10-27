use crate::base::state::Base;
use dominator::{clone, html, with_node, Dom};
use std::rc::Rc;
use shared::domain::jig::module::body::legacy::activity::Activity;
use super::{
    say_something::SaySomething,
    soundboard::Soundboard,
};

impl Base {
    pub fn render_activity(self: Rc<Self>) -> Dom {
        match self.slide.activity.clone() {
            Some(activity) => match activity {
                Activity::SaySomething(activity) => {
                    SaySomething::new(self.clone(), activity).render()
                },
                Activity::Soundboard(activity) => {
                    Soundboard::new(self.clone(), activity).render()
                },
                _ => html!("empty-fragment"),
            },
            None => html!("empty-fragment"),
        }
    }
}