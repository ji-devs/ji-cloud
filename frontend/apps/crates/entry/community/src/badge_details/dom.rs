use std::rc::Rc;

use dominator::{Dom, html};

use super::BadgeDetails;

impl BadgeDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_badge();

        html!("div", {
            .text("badges")
            .child_signal(state.badge.signal_ref(move |badge| {
                badge.as_ref().map(|badge| {
                    html!("p", {
                        .text(&badge.display_name)
                    })
                })
            }))
        })
    }
}
