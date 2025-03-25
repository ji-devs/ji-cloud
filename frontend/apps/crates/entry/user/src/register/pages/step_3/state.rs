use crate::register::state::Step2Data;
use dominator_helpers::futures::AsyncLoader;
use std::cell::RefCell;
use std::collections::HashSet;

pub struct State {
    pub step_2: Step2Data,
    pub register_loader: AsyncLoader,
    pub affiliations: RefCell<HashSet<String>>,
    pub age_ranges: RefCell<HashSet<String>>,
    pub subjects: RefCell<HashSet<String>>,
}

impl State {
    pub fn new(step_2: Step2Data) -> Self {
        Self {
            step_2,
            affiliations: RefCell::new(HashSet::new()),
            age_ranges: RefCell::new(HashSet::new()),
            subjects: RefCell::new(HashSet::new()),
            register_loader: AsyncLoader::new(),
        }
    }
}
