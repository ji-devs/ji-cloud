use std::{collections::HashSet, rc::Rc};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{
    pro_dev::{ProDevId, ProDevResponse, unit::{ProDevUnitId, ProDevUnit}},
    meta::ResourceType,
};
use utils::asset::ProDevPlayerOptions;

pub struct ProDevPlayer {
    pub pro_dev_id: ProDevId,
    pub pro_dev: Mutable<Option<ProDevResponse>>,
    pub units: Mutable<Vec<ProDevUnit>>,
    pub units_done: Mutable<HashSet<ProDevUnitId>>,
    pub loader: AsyncLoader,
    pub played_units: Mutable<HashSet<ProDevUnitId>>,
    pub player_options: ProDevPlayerOptions,
    pub active_unit: Mutable<Option<ProDevUnitId>>,
    pub resource_types: Mutable<Vec<ResourceType>>,
}

impl ProDevPlayer {
    pub fn new(pro_dev_id: ProDevId, player_options: ProDevPlayerOptions) -> Rc<Self> {
        Rc::new(Self {
            pro_dev_id,
            pro_dev: Mutable::new(None),
            units: Mutable::new(vec![]),
            units_done: Default::default(),
            loader: AsyncLoader::new(),
            played_units: Mutable::new(HashSet::new()),
            player_options,
            active_unit: Mutable::new(None),
            resource_types: Default::default(),
        })
    }
}
