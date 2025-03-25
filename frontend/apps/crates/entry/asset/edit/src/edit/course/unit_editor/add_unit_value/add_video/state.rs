use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::module::body::_groups::design::YoutubeEmbed;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddVideo {
    pub video: Mutable<Option<YoutubeEmbed>>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
    pub loader: AsyncLoader,
}

impl AddVideo {
    pub fn new(
        add_unit_value_state: Rc<AddUnitValueState>,
        video: &Option<YoutubeEmbed>,
    ) -> Rc<Self> {
        Rc::new(Self {
            video: Mutable::new(video.clone()),
            add_unit_value_state,
            loader: AsyncLoader::new(),
        })
    }
}
