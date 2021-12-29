use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal_vec::MutableVec, signal::Mutable};
use shared::domain::{jig::JigResponse, meta::{MetadataResponse, AgeRange, Goal, AgeRangeId, GoalId, AffiliationId, Affiliation}};
use std::{rc::Rc, collections::HashMap};

pub struct JigUI {
    pub jigs: MutableVec<JigResponse>,
    pub loader: AsyncLoader,
    pub meta: Mutable<Option<MetadataResponse>>,
    pub goals: Mutable<HashMap<GoalId, Goal>>,
    pub ages: Mutable<HashMap<AgeRangeId, AgeRange>>,
    pub affiliations: Mutable<HashMap<AffiliationId, Affiliation>>,
}

impl JigUI {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            jigs: MutableVec::new(),
            loader: AsyncLoader::new(),
            meta: Mutable::new(None),
            goals: Mutable::new(HashMap::new()),
            ages: Mutable::new(HashMap::new()),
            affiliations: Mutable::new(HashMap::new()),
        })
    }
}
