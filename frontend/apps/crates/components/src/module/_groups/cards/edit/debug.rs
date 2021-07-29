use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use once_cell::sync::OnceCell;
use utils::{prelude::*, colors::*};
use uuid::Uuid;
use shared::{
    domain::{
        jig::{
            module::body::_groups::cards::Step,
            JigId, module::ModuleId
        },
        image::ImageId,
        audio::AudioId
    },
    media::MediaLibrary
};
pub use super::sidebar::{
    step_1::state::TabKind as Step1TabKind, 
    step_2::state::TabKind as Step2TabKind, 
    step_3::state::TabKind as Step3TabKind, 
};

#[derive(Debug, Default, Clone)]
pub struct DebugSettings {
    pub step1_tab: Option<Step1TabKind>,
    pub step2_tab: Option<Step2TabKind>,
    pub step3_tab: Option<Step3TabKind>,
}
