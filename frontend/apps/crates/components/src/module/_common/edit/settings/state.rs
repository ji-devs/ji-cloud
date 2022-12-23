use super::{button::state::SettingsButton, line::state::LineKind};
use std::rc::Rc;

pub struct ModuleSettings {
    pub lines: Vec<ModuleSettingsLine>,
}

pub struct ModuleSettingsLine {
    pub line_type: ModuleSettingsLineType,
    pub settings: Vec<Option<Rc<SettingsButton>>>,
}

pub enum ModuleSettingsLineType {
    Kind(LineKind),
    Label(String),
}

impl ModuleSettingsLine {
    pub fn new(kind: LineKind, settings: Vec<Option<Rc<SettingsButton>>>) -> Self {
        Self {
            line_type: ModuleSettingsLineType::Kind(kind),
            settings,
        }
    }

    pub fn new_with_label(label: String, settings: Vec<Option<Rc<SettingsButton>>>) -> Self {
        Self {
            line_type: ModuleSettingsLineType::Label(label),
            settings,
        }
    }
}
