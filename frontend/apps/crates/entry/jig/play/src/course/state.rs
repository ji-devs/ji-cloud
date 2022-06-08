use std::{collections::HashSet, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    asset::DraftOrLive,
    course::{CourseId, CourseResponse},
    jig::{JigId, JigResponse},
};

pub struct CoursePlayer {
    pub course_id: CourseId,
    pub course: Mutable<Option<CourseResponse>>,
    pub jigs: Mutable<Vec<JigResponse>>,
    pub loader: AsyncLoader,
    pub played_jigs: Mutable<HashSet<JigId>>,
    // pub player_options: JigPlayerOptions,
    pub active_jig: Mutable<Option<JigId>>,
    pub draft_or_live: DraftOrLive,
}

impl CoursePlayer {
    pub fn new(
        course_id: CourseId,
        // player_options: JigPlayerOptions,
    ) -> Rc<Self> {
        Rc::new(Self {
            course_id,
            course: Mutable::new(None),
            jigs: Mutable::new(vec![]),
            loader: AsyncLoader::new(),
            played_jigs: Mutable::new(HashSet::new()),
            // player_options,
            active_jig: Mutable::new(None),
            draft_or_live: DraftOrLive::Live, // TODO: take from url
        })
    }
}
