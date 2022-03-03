use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::meta::{Affiliation, AgeRange};
use utils::routes::AdminCurationRoute;

use super::jig::state::jig::EditableJig;

pub struct Curation {
    pub route: Mutable<AdminCurationRoute>,
    pub jigs: MutableVec<Rc<EditableJig>>,
    pub fetch_mode: RefCell<FetchMode>,
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
    Search(String),
}
