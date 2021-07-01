use super::{line::state::LineKind, button::state::SettingsButton};
use std::rc::Rc;

pub struct ModuleSettings {
    pub lines: Vec<(LineKind, Vec<Rc<SettingsButton>>)>
}
