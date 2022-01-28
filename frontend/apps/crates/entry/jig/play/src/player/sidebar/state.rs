use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::{jig::report::JigReportType, meta::AgeRange};

use super::super::state::State as PlayerState;

pub struct State {
    pub sidebar_open: Mutable<bool>,
    pub player_state: Rc<PlayerState>,
    pub info_popup_active: Mutable<bool>,
    pub report_status: Mutable<ReportStatus>,
    pub report_type: Mutable<Option<JigReportType>>,
    pub all_ages: Mutable<Vec<AgeRange>>,
    pub loader: AsyncLoader,
}

impl State {
    pub fn new(player_state: Rc<PlayerState>) -> Self {
        Self {
            info_popup_active: Mutable::new(false),
            sidebar_open: Mutable::new(false),
            player_state,
            report_status: Mutable::new(ReportStatus::Default),
            report_type: Mutable::new(None),
            all_ages: Mutable::new(vec![]),
            loader: AsyncLoader::new(),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Default,
    Active,
    Sent,
}
