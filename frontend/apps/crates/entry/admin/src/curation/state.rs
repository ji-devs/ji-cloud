use std::rc::Rc;

use awsm_web::loaders::helpers::AsyncLoader;
use futures_signals::{signal_vec::MutableVec, signal::Mutable};
use shared::domain::{jig::JigResponse, meta::{Goal, AgeRange, Affiliation}};
use utils::routes::AdminCurationRoute;

pub struct Curation {
    pub route: Mutable<AdminCurationRoute>,
    pub jigs: MutableVec<JigResponse>,
    pub goals: Mutable<Vec<Goal>>,
    pub loader: AsyncLoader,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
}

impl Curation {
    pub fn new(route: AdminCurationRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            jigs: MutableVec::new(),
            loader: AsyncLoader::new(),
            goals: Mutable::new(Vec::new()),
            ages: Mutable::new(Vec::new()),
            affiliations: Mutable::new(Vec::new()),
        })
    }
}
