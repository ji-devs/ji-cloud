use std::rc::Rc;
use chrono::{DateTime, Utc};

use futures_signals::signal::Mutable;
use shared::domain::admin::ExportType;

pub struct Export {
    pub export_type: Mutable<Option<ExportType>>,
    pub from_date: Mutable<Option<DateTime<Utc>>>,
    pub to_date: Mutable<Option<DateTime<Utc>>>,
}

impl Export {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            export_type: Mutable::new(Some(ExportType::Profiles)),
            from_date: Mutable::new(None),
            to_date: Mutable::new(None),
        })
    }
}
