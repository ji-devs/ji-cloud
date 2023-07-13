use dominator::clone;
use shared::{
    api::endpoints::course,
    domain::{
        asset::DraftOrLive,
        course::{CourseGetDraftPath, CourseGetLivePath},
    },
};
use std::rc::Rc;
use utils::{paywall, prelude::ApiEndpointExt};

use super::state::CoursePlayer;

impl CoursePlayer {
    pub fn load_data(self: &Rc<Self>) {
        let state = self;

        if !paywall::can_play_course() {
            paywall::dialog_play(
                "
                    Looking to view a course?
                    Upgrade now for UNLIMITED JIGs and resources.
                ",
            );
            return;
        }

        state.loader.load(clone!(state => async move {
            let course = match state.player_options.draft_or_live {
                DraftOrLive::Live => {
                    let course = {
                        course::GetLive::api_no_auth(CourseGetLivePath(state.course_id), None).await
                    };

                    course
                },
                DraftOrLive::Draft => {
                    let course = {
                        course::GetDraft::api_no_auth(CourseGetDraftPath(state.course_id), None).await
                    };

                    course
                },
            };

            match course {
                Ok(course) => {
                    if let Some(start_unit_id) = state.start_unit_id {
                        if let Some((index, _)) = course.course_data.units.iter().enumerate().find(|unit| {
                            unit.1.id == start_unit_id
                        }) {
                            state.active_unit.set_neq(Some(index));
                        };
                    }
                    state.course.set(Some(Rc::new(course)));
                },
                Err(_) => {
                    todo!();
                },
            }
        }));
    }
}
