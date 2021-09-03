use crate::tabs::MenuTabKind;

#[derive(Debug, Default, Clone)]
pub struct DebugSettings {
    pub step1_tab: Option<MenuTabKind>,
    pub step2_tab: Option<MenuTabKind>,
    pub step3_tab: Option<MenuTabKind>,
}
