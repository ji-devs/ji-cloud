use std::rc::Rc;

use dominator::{Dom, html};

use super::MemberDetails;

impl MemberDetails {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_member();

        html!("div", {
            .text("member details")
            // .child_signal(state.member.signal_ref(move |member| {
            //     member.as_ref().map(|member| {
            //         html!("p", {
            //             .text(&member.display_name)
            //         })
            //     })
            // }))
        })
    }
}
