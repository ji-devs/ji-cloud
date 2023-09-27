use chrono::{DateTime, Utc};
use std::{fmt::Display, rc::Rc};

use futures_signals::signal::Mutable;
use shared::domain::admin::DateFilterType;

pub struct Export {
    pub export_type: Mutable<Option<ExportType>>,
    pub date_filter_type: Mutable<DateFilterType>,
    pub from_date: Mutable<Option<DateTime<Utc>>>,
    pub to_date: Mutable<Option<DateTime<Utc>>>,
}

impl Export {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            export_type: Mutable::new(None),
            date_filter_type: Mutable::new(Default::default()),
            from_date: Mutable::new(None),
            to_date: Mutable::new(None),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, strum_macros::EnumIter)]
pub enum ExportType {
    Users,
    Jigs,
    Playlists,
}

impl Display for ExportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExportType::Users => write!(f, "Users"),
            ExportType::Jigs => write!(f, "Jigs"),
            ExportType::Playlists => write!(f, "Playlists"),
        }
    }
}
