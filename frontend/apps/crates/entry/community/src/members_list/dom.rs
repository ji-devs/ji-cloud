use std::rc::Rc;

use dominator::{Dom, html};

use super::MembersList;

impl MembersList {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        state.load_members();

        html!("div", {
            .text("members")
            // .children_signal_vec(state.members.signal_vec_cloned().map(clone!(state => move |member| {
            //     state.render_member(member)
            // })))
        })
    }

    // fn render_member(self: &Rc<Self>, member: Member) -> Dom {
    //     html!("p", {
    //         .text(&member.display_name)
    //     })
    // }
}
