use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::course::{CourseId, CourseResponse, CourseUpdateDraftDataRequest};

pub struct State {
    pub popup_open: Mutable<bool>,
    pub play_in_order: Mutable<bool>,
    pub course_id: CourseId,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(course: &CourseResponse) -> Self {
        Self {
            popup_open: Mutable::new(false),
            play_in_order: Mutable::new(false),
            course_id: course.id,
            loader: AsyncLoader::new(),
        }
    }

    pub fn get_course_update_req(&self) -> CourseUpdateDraftDataRequest {
        CourseUpdateDraftDataRequest {
            ..Default::default()
        }
    }

    // fn get_player_settings(&self) -> CoursePlayerSettings {
    //     CoursePlayerSettings {
    //         direction: self.direction.get(),
    //         display_score: self.display_score.get(),
    //         track_assessments: self.track_assessments.get(),
    //         drag_assist: self.drag_assist.get(),
    //     }
    // }
}
