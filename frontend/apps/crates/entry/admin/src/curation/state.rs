use std::{rc::Rc, cell::RefCell};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal_vec::MutableVec, signal::Mutable};
use shared::domain::{jig::JigResponse, meta::{Goal, AgeRange, Affiliation}};
use utils::routes::AdminCurationRoute;

pub struct Curation {
    pub route: Mutable<AdminCurationRoute>,
    pub jigs: MutableVec<JigResponse>,
    pub fetch_mode: RefCell<FetchMode>,
    pub goals: Mutable<Vec<Goal>>,
    pub loader: AsyncLoader,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub active_page: Mutable<u32>,
    pub total_pages: Mutable<Option<u32>>,
}

impl Curation {
    pub fn new(route: AdminCurationRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            jigs: MutableVec::new(),
            fetch_mode: RefCell::new(FetchMode::Browse),
            loader: AsyncLoader::new(),
            goals: Mutable::new(Vec::new()),
            ages: Mutable::new(Vec::new()),
            affiliations: Mutable::new(Vec::new()),
            active_page: Mutable::new(0),
            total_pages: Mutable::new(None),
        })
    }
}

#[derive(Clone, Debug)]
pub enum FetchMode {
    Browse,
    Search(String)
}
