use std::{collections::HashSet, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::pro_dev::{unit::ProDevUnitId, ProDevId, ProDevResponse};
use utils::asset::ProDevPlayerOptions;

pub struct ProDevPlayer {
    pub pro_dev_id: ProDevId,
    /// Loaded after [`State`] is initialized necessitating an Option
    pub pro_dev: Mutable<Option<Rc<ProDevResponse>>>,
    pub loader: AsyncLoader,
    pub active_unit: Mutable<Option<usize>>,
    pub played_units: Mutable<HashSet<usize>>,
    pub current_page: Mutable<Option<usize>>, // TODO: what is this??
    pub start_unit_id: Option<ProDevUnitId>,
    pub player_options: ProDevPlayerOptions,
}

impl ProDevPlayer {
    pub fn new(
        pro_dev_id: ProDevId,
        unit_id: Option<ProDevUnitId>,
        player_options: ProDevPlayerOptions,
    ) -> Rc<Self> {
        Rc::new(Self {
            pro_dev_id,
            pro_dev: Default::default(),
            loader: AsyncLoader::new(),
            active_unit: Default::default(),
            played_units: Default::default(),
            start_unit_id: unit_id,
            current_page: Mutable::new(None),
            player_options,
        })
    }
}
