pub use super::sidebar::{
    step_1::state::TabKind as Step1TabKind, step_2::state::TabKind as Step2TabKind,
    step_3::state::TabKind as Step3TabKind,
};

#[derive(Debug, Default, Clone)]
pub struct DebugSettings {
    pub step1_tab: Option<Step1TabKind>,
    pub step2_tab: Option<Step2TabKind>,
    pub step3_tab: Option<Step3TabKind>,
}
