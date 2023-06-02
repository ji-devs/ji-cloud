use std::{collections::HashSet, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::course::{unit::CourseUnitId, CourseId, CourseResponse};
use utils::asset::CoursePlayerOptions;

pub struct CoursePlayer {
    pub course_id: CourseId,
    /// Loaded after [`State`] is initialized necessitating an Option
    pub course: Mutable<Option<Rc<CourseResponse>>>,
    pub loader: AsyncLoader,
    pub active_unit: Mutable<Option<usize>>,
    pub played_units: Mutable<HashSet<usize>>,
    pub current_page: Mutable<Option<usize>>, // TODO: what is this??
    pub start_unit_id: Option<CourseUnitId>,
    pub player_options: CoursePlayerOptions,
}

impl CoursePlayer {
    pub fn new(
        course_id: CourseId,
        unit_id: Option<CourseUnitId>,
        player_options: CoursePlayerOptions,
    ) -> Rc<Self> {
        Rc::new(Self {
            course_id,
            course: Default::default(),
            loader: AsyncLoader::new(),
            active_unit: Default::default(),
            played_units: Default::default(),
            start_unit_id: unit_id,
            current_page: Mutable::new(None),
            player_options,
        })
    }
}
