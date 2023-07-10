use std::rc::Rc;

use dominator::{clone, html, Dom};
use utils::routes::AdminCourseCurationRoute;

use crate::course_curation::table::state::CourseTable;

use super::CourseCuration;

impl CourseCuration {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.loader.is_loading())
            }))
            .child_signal(self.route.signal_ref(clone!(state => move|route| {
                Some(match route {
                    AdminCourseCurationRoute::Table => {
                        CourseTable::new(
                            Rc::clone(&state)
                        ).render()
                    },
                    AdminCourseCurationRoute::Course(_course_id) => {
                        html!("empty-fragment", {
                            .child(html!("div", {
                                .text("TODO")
                            }))
                            // .child_signal(from_future(state.clone().get_course(*course_id)).map(clone!(state => move|course| {
                            //     course.map(|course| {
                            //         CourseDetails::new(
                            //             Rc::clone(&state),
                            //             course.id,
                            //             course
                            //         ).render()
                            //     })
                            // })))
                        })
                    },
                })
            })))
        })
    }
}
