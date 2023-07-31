use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::billing::AdminSchool;
use shared::domain::ItemCount;
use std::rc::Rc;

use crate::schools::Schools;

pub struct SchoolTable {
    pub parent: Rc<Schools>,
    pub schools: MutableVec<Rc<AdminSchool>>,
    pub total_pages: Mutable<Option<ItemCount>>,
    pub table_state: Mutable<TableState>,
    pub uploading: Mutable<bool>,
}

impl SchoolTable {
    pub fn new(parent: Rc<Schools>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            table_state: Mutable::new(TableState::Table),
            uploading: Mutable::new(false),
            schools: MutableVec::new(),
            total_pages: Mutable::new(None),
        })
    }
}

#[derive(Clone, Debug)]
pub enum TableState {
    Table,
    UploadResults(Vec<String>),
}
