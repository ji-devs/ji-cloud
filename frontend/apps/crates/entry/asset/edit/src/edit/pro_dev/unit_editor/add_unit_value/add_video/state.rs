use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::module::body::_groups::design::{YoutubeEmbed, YoutubeUrl};

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddVideo {
    pub video: Mutable<Option<YoutubeEmbed>>,
    pub url_str: Mutable<String>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
    pub loader: AsyncLoader,
}

impl AddVideo {
    pub fn new(
        add_unit_value_state: Rc<AddUnitValueState>,
        video: &Option<YoutubeEmbed>,
    ) -> Rc<Self> {
        let url_str = match video {
            Some(video) => {
                let YoutubeUrl(url) = &video.url;
                url.to_string()
            }
            None => String::new(),
        };

        Rc::new(Self {
            video: Mutable::new(video.clone()),
            url_str: Mutable::new(url_str),
            add_unit_value_state,
            loader: AsyncLoader::new(),
        })
    }
}
