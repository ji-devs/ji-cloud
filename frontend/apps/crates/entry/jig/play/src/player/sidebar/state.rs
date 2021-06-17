use std::rc::Rc;

use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;
use futures_signals::signal::Mutable;
use utils::unwrap::UnwrapJiExt;

use super::super::state::State as PlayerState;

const STR_REPORT_TYPE_OFFENSIVE: &'static str = "Offensive";
const STR_REPORT_TYPE_COPYRIGHT: &'static str = "Copyright Infringement";
const STR_REPORT_TYPE_SPAM: &'static str = "Spam";
const STR_REPORT_TYPE_PRIVACY: &'static str = "Privacy";
const STR_REPORT_TYPE_OTHER: &'static str = "Other";


pub struct State {
    pub active_popup: Mutable<ActivePopup>,
    pub sidebar_open: Mutable<bool>,
    pub player_state: Rc<PlayerState>,
    pub report_status: Mutable<ReportStatus>,
    pub report_type: Mutable<Option<ReportType>>,
}

impl State {
    pub fn new(player_state: Rc<PlayerState>) -> Self {
        Self {
            active_popup: Mutable::new(ActivePopup::None),
            sidebar_open: Mutable::new(false),
            player_state,
            report_status: Mutable::new(ReportStatus::Default),
            report_type: Mutable::new(None)
        }
    }
}

#[derive(Clone)]
pub enum ActivePopup {
    None,
    ShareMain,
    ShareStudents,
    ShareEmbed,
    JigInfo,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ReportStatus {
    Default,
    Active,
    Sent,
}


#[derive(Serialize, Deserialize, EnumIter, Debug)]
pub enum ReportType {
    Offensive,
    CopyrightInfringement,
    Spam,
    Privacy,
    Other,
}

impl ReportType {
    pub fn to_locale_str(&self) -> &'static str {
        match self {
            ReportType::Offensive => STR_REPORT_TYPE_OFFENSIVE,
            ReportType::CopyrightInfringement => STR_REPORT_TYPE_COPYRIGHT,
            ReportType::Spam => STR_REPORT_TYPE_SPAM,
            ReportType::Privacy => STR_REPORT_TYPE_PRIVACY,
            ReportType::Other => STR_REPORT_TYPE_OTHER,
        }
    }

    pub fn to_value_str(&self) -> String {
        serde_json::to_string(&self).unwrap_ji()
    }

    pub fn from_value_str(s: &str) -> Self {
        serde_json::from_str(s).unwrap_ji()
    }
}
