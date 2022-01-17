use super::{
    ask_questions::AskQuestions, say_something::SaySomething, soundboard::Soundboard, video::Video, talk_type::TalkType, puzzle::Puzzle,
};
use crate::base::state::Base;
use dominator::{html, Dom};
use shared::domain::jig::module::body::legacy::activity::Activity;
use std::rc::Rc;

impl Base {
    pub fn render_activity(self: Rc<Self>) -> Dom {
        match self.slide.activity.clone() {
            Some(activity) => match activity {
                Activity::SaySomething(activity) => {
                    SaySomething::new(self.clone(), activity).render()
                }
                Activity::Soundboard(activity) => Soundboard::new(self.clone(), activity).render(),
                Activity::Video(activity) => Video::new(self.clone(), activity).render(),
                Activity::AskQuestions(activity) => {
                    if activity.items.len() > 0 {
                        AskQuestions::new(self.clone(), activity).render()
                    } else {
                        self.allow_stage_click();
                        html!("empty-fragment")
                    }
                },
                Activity::TalkType(activity) => TalkType::new(self.clone(), activity).render(),
                Activity::Puzzle(activity) => Puzzle::new(self.clone(), activity).render(),
                // _ => html!("empty-fragment"),
            },
            None => {
                self.allow_stage_click();
                html!("empty-fragment")
            },
        }
    }
}
