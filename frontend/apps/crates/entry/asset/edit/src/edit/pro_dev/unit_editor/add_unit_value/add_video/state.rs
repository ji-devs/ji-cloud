use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::pro_dev::unit::Video;

use super::super::state::AddUnitValue as AddUnitValueState;

pub struct AddVideo {
    pub video: Mutable<Option<Video>>,
    pub url_str: Mutable<String>,
    pub add_unit_value_state: Rc<AddUnitValueState>,
}

impl AddVideo {
    pub fn new(add_unit_value_state: Rc<AddUnitValueState>, video: &Option<Video>) -> Rc<Self> {

        let url_str = if let Some(url) = video {
            url.host.get_url_string()
        } else {
            "".to_string()
        };
        
        Rc::new(Self {
            video: Mutable::new(video.clone()),
            url_str: Mutable::new(url_str),
            add_unit_value_state,
        })
    }
    
}
