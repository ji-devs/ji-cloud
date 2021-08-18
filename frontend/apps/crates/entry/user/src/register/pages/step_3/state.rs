use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step2Data};
use std::collections::HashSet;
use std::cell::RefCell;
use dominator_helpers::futures::AsyncLoader;
use utils::{events, routes::*, api_helpers::meta::MetaOptions};

pub struct State {
    pub step: Mutable<Step>,
    pub step_2: Step2Data,
    pub register_loader: AsyncLoader,
    pub affiliations: RefCell<HashSet<String>>,
    pub age_ranges: RefCell<HashSet<String>>,
    pub subjects: RefCell<HashSet<String>>,
}

impl State {
    pub fn new(step: Mutable<Step>, step_2: Step2Data) -> Self {
        Self {
            step,
            step_2,
            affiliations: RefCell::new(HashSet::new()),
            age_ranges: RefCell::new(HashSet::new()),
            subjects: RefCell::new(HashSet::new()),
            register_loader: AsyncLoader::new(),
        }
    }

}
