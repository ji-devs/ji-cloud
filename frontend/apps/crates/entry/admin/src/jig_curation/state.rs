use std::{cell::RefCell, rc::Rc};

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    asset::OrderBy,
    meta::{Affiliation, AgeRange},
};
use utils::{editable_asset::EditableJig, routes::AdminJigCurationRoute};

pub struct JigCuration {
    pub route: Mutable<AdminJigCurationRoute>,
    pub jigs: MutableVec<Rc<EditableJig>>,
    pub fetch_mode: RefCell<FetchMode>,
    pub loader: AsyncLoader,
    pub ages: Mutable<Vec<AgeRange>>,
    pub affiliations: Mutable<Vec<Affiliation>>,
    pub active_page: Mutable<u32>,
    pub total_pages: Mutable<Option<u32>>,
    pub order_by: Mutable<OrderBy>,
}

impl JigCuration {
    pub fn new(route: AdminJigCurationRoute) -> Rc<Self> {
        Rc::new(Self {
            route: Mutable::new(route),
            jigs: MutableVec::new(),
            fetch_mode: RefCell::new(FetchMode::Browse),
            loader: AsyncLoader::new(),
            ages: Mutable::new(Vec::new()),
            affiliations: Mutable::new(Vec::new()),
            active_page: Mutable::new(0),
            total_pages: Mutable::new(None),
            order_by: Mutable::new(OrderBy::PublishedAt),
        })
    }
}

#[derive(Clone, Debug)]
pub enum FetchMode {
    Browse,
    Search(String),
}
