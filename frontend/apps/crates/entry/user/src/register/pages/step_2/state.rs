use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step1Data};
use std::cell::RefCell;

pub struct State {
    pub step: Mutable<Step>,
    pub step_1: Step1Data,
    pub location_json: RefCell<Option<String>>,
    pub language: RefCell<String>,
    pub terms: RefCell<bool>,
    pub terms_status: Mutable<Option<TermsError>>,
    pub marketing: RefCell<bool>,
}

impl State {
    pub fn new(step: Mutable<Step>, step_1: Step1Data) -> Self {
        Self {
            step,
            step_1,
            location_json: RefCell::new(None),
            language: RefCell::new("".to_string()),
            terms: RefCell::new(false),
            terms_status: Mutable::new(None), 
            marketing: RefCell::new(false),
        }
    }

    pub fn clear_terms_status(&self) {
        self.terms_status.set(None);
    }

    pub fn terms_error(&self) -> impl Signal<Item = &'static str> {
        self.terms_status
            .signal_cloned()
            .map(|err| {
                err
                    .map(|err| err.as_str())
                    .unwrap_or("")
            })
    }
}


#[derive(Debug, Clone)]
pub enum TermsError {
    Unchecked,
}

impl TermsError {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unchecked => "need to select!",
        }
    }
}
