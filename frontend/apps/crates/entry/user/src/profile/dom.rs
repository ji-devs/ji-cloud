use std::rc::Rc;
use dominator::{html, Dom};
use super::{actions, state::State};

pub struct ProfilePage {
}

impl ProfilePage {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        state.loader.load(actions::load_profile(state.clone()));

        html!("div", {
            .text_signal(state.status.signal_ref(|status| {
                status
                    .as_ref()
                    .map(|status| match status {
                        Ok(profile) => format!("{:#?}", profile),
                        Err(_) => "not logged in!".to_string()
                    })
                    .unwrap_or("loading...".to_string())
            }))
        })
    }

}
