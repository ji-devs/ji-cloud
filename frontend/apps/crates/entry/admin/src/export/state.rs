use shared::{DateTime, Utc};
use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::admin::{DateFilterType, ExportType};

pub struct Export {
    pub export_type: Mutable<ExportType>,
    pub date_filter_type: Mutable<DateFilterType>,
    pub from_date: Mutable<Option<DateTime<Utc>>>,
    pub to_date: Mutable<Option<DateTime<Utc>>>,
}

impl Export {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            export_type: Mutable::new(Default::default()),
            date_filter_type: Mutable::new(Default::default()),
            from_date: Mutable::new(None),
            to_date: Mutable::new(None),
        })
    }
}
