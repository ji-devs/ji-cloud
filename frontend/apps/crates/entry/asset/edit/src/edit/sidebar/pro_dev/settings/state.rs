use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::pro_dev::ProDevUpdateDraftDataRequest;
use utils::editable_asset::EditableProDev;

pub struct ProDevSettings {
    pub pro_dev: Rc<EditableProDev>,
    pub popup_open: Mutable<bool>,
    pub play_in_order: Mutable<bool>,
    pub loader: AsyncLoader,
}

impl ProDevSettings {
    pub fn new(pro_dev: &Rc<EditableProDev>) -> Rc<Self> {
        Rc::new(Self {
            pro_dev: Rc::clone(pro_dev),
            popup_open: Mutable::new(false),
            play_in_order: Mutable::new(false),
            loader: AsyncLoader::new(),
        })
    }

    pub fn get_pro_dev_update_req(&self) -> ProDevUpdateDraftDataRequest {
        ProDevUpdateDraftDataRequest {
            ..Default::default()
        }
    }

    // fn get_player_settings(&self) -> ProDevPlayerSettings {
    //     ProDevPlayerSettings {
    //         direction: self.direction.get(),
    //         display_score: self.display_score.get(),
    //         track_assessments: self.track_assessments.get(),
    //         drag_assist: self.drag_assist.get(),
    //     }
    // }
}
