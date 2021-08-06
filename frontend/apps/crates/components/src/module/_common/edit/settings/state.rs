use super::{button::state::SettingsButton, line::state::LineKind};
use std::rc::Rc;

pub struct ModuleSettings {
    pub lines: Vec<(LineKind, Vec<Option<Rc<SettingsButton>>>)>,
}
