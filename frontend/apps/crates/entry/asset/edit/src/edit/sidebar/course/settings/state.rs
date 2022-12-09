use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::course::CourseUpdateDraftDataRequest;
use utils::editable_asset::EditableCourse;

pub struct CourseSettings {
    pub course: Rc<EditableCourse>,
    pub popup_open: Mutable<bool>,
    pub play_in_order: Mutable<bool>,
    pub loader: AsyncLoader,
}

impl CourseSettings {
    pub fn new(course: &Rc<EditableCourse>) -> Rc<Self> {
        Rc::new(Self {
            course: Rc::clone(course),
            popup_open: Mutable::new(false),
            play_in_order: Mutable::new(false),
            loader: AsyncLoader::new(),
        })
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
