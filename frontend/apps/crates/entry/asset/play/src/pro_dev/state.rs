use std::{cell::RefCell, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use components::audio::mixer::AudioHandle;
use futures_signals::signal::Mutable;
use serde::{Deserialize, Serialize};
use shared::domain::{
    pro_dev::{ProDevId, ProDevResponse,unit::{
        ProDevUnitId,
    }},
    meta::ResourceType,

};
use utils::asset::ProDevPlayerOptions;
use web_sys::HtmlIFrameElement;


pub struct ProDevPlayer {
    pub pro_dev_id: ProDevId,
    pub pro_dev: Mutable<Option<ProDevResponse>>,
    /// Loaded after [`State`] is initialized necessitating an Option
    pub pro_dev_liked: Mutable<Option<bool>>,
    pub loader: AsyncLoader,
    pub active_unit: Mutable<Option<usize>>,
    // /// Count of units which have been played
    // pub played_units: RefCell<usize>,
    pub play_tracked: RefCell<bool>,
    pub start_unit_id: Option<ProDevUnitId>,
    pub player_options: ProDevPlayerOptions,
    pub resource_types: Mutable<Vec<ResourceType>>,
    pub is_full_screen: Mutable<bool>,
}

impl ProDevPlayer {
    pub fn new(
        pro_dev_id: ProDevId,
        unit_id: Option<ProDevUnitId>,
        player_options: ProDevPlayerOptions,
    ) -> Rc<Self> {
        let active_unit = match unit_id {
            // If the unit_id is specified, then we need to make sure that we don't unecessarily load
            // the first unit;
            Some(_) => Mutable::new(None),
            // Otherwise, if no unit_id is set, then set the active unit to the first unit.
            None => Mutable::new(Some(0)),
        };

        Rc::new(Self {
            pro_dev_id,
            pro_dev: Mutable::new(None),
            pro_dev_liked: Mutable::new(None),
            loader: AsyncLoader::new(),
            active_unit,
            // played_units: RefCell::new(0),
            play_tracked: RefCell::new(false),
            start_unit_id: unit_id,
            player_options,
            resource_types: Default::default(),
            is_full_screen: Mutable::new(false),
        })
    }
}



/// Returns whether the liked status should be loaded for a JIG
///
/// Returns true only if there is a logged-in user who is **not** the author of the JIG, and the
/// JIG is published.
pub fn can_load_liked_status(pro_dev: &ProDevResponse) -> bool {
    match utils::init::user::get_user_id() {
        Some(user_id) if pro_dev.pro_dev_data.draft_or_live.is_live() => match pro_dev.author_id {
            Some(author_id) => author_id != user_id,
            None => true,
        },
        _ => false, // No logged-in user
    }
}