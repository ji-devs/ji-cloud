use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{jig::report::JigReportType, meta::AgeRange};

use super::super::state::JigPlayer;

pub struct Sidebar {
    pub sidebar_open: Mutable<bool>,
    pub player_state: Rc<JigPlayer>,
    pub info_popup_active: Mutable<bool>,
    pub report_status: Mutable<ReportStatus>,
    pub report_type: Mutable<Option<JigReportType>>,
    pub all_ages: Mutable<Vec<AgeRange>>,
    pub loader: AsyncLoader,
}

impl Sidebar {
    pub fn new(player_state: &Rc<JigPlayer>) -> Rc<Self> {
        Rc::new(Self {
            info_popup_active: Mutable::new(false),
            sidebar_open: Mutable::new(false),
            player_state: Rc::clone(player_state),
            report_status: Mutable::new(ReportStatus::Default),
            report_type: Mutable::new(None),
            all_ages: Mutable::new(vec![]),
            loader: AsyncLoader::new(),
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Default,
    Active,
    Sent,
}
