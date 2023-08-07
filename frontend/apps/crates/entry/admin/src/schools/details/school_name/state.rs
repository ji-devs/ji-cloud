use crate::schools::details::state::SchoolDetails;
use futures_signals::map_ref;
use futures_signals::signal::{Mutable, Signal};
use shared::domain::billing::SchoolName;
use std::rc::Rc;
use utils::editable_field::{EditableField, Nullable};

pub struct SchoolNameState {
    pub parent: Rc<SchoolDetails>,
    pub new_name: EditableField<Nullable<String>>,
    pub current_name: EditableField<Nullable<SchoolName>>,
    pub filter_value: Mutable<String>,
    pub school_names: Mutable<Option<Vec<SchoolName>>>,
}

impl SchoolNameState {
    pub fn new(parent: Rc<SchoolDetails>, current_name: Option<SchoolName>) -> Rc<Self> {
        Rc::new(Self {
            parent,
            new_name: Default::default(),
            current_name: current_name.into(),
            filter_value: Default::default(),
            school_names: Default::default(),
        })
    }

    pub fn changed_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let new_name = self.new_name.signal(),
            let current_name = self.current_name.changed_signal()
            => {
                match new_name {
                    Some(_) => true,
                    None => *current_name,
                }
            }
        }
    }
}
