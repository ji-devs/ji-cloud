use std::{rc::Rc, collections::HashMap};

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::{signal_vec::MutableVec, signal::Mutable};
use shared::domain::{jig::JigResponse, meta::{GoalId, Goal, AgeRangeId, AgeRange, AffiliationId, Affiliation}};
use utils::routes::AdminCurationRoute;

pub struct Curation {
    pub route: Mutable<AdminCurationRoute>,
    pub jigs: MutableVec<JigResponse>,
    pub goals: Mutable<HashMap<GoalId, Goal>>,
    pub loader: AsyncLoader,
    pub ages: Mutable<HashMap<AgeRangeId, AgeRange>>,
    pub affiliations: Mutable<HashMap<AffiliationId, Affiliation>>,
}

impl Curation {
    pub fn new(route: AdminCurationRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            jigs: MutableVec::new(),
            loader: AsyncLoader::new(),
            goals: Mutable::new(HashMap::new()),
            ages: Mutable::new(HashMap::new()),
            affiliations: Mutable::new(HashMap::new()),
        })
    }
}
