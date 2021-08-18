use futures_signals::signal::{Mutable, Signal, SignalExt};
use crate::register::state::{Step, Step1Data};
use std::cell::RefCell;

pub struct State {
    pub step: Mutable<Step>,
    pub step_1: Step1Data,
    pub location_json: RefCell<Option<String>>,
    pub location_error: Mutable<bool>,
    pub language: RefCell<Option<String>>,
    pub language_error: Mutable<bool>,
    pub persona: RefCell<Option<String>>,
    pub persona_error: Mutable<bool>,
    pub organization: RefCell<Option<String>>,
    pub organization_error: Mutable<bool>,
    pub terms: RefCell<bool>,
    pub terms_error: Mutable<Option<TermsError>>,
    pub marketing: RefCell<bool>,
}

impl State {
    pub fn new(step: Mutable<Step>, step_1: Step1Data) -> Self {
        Self {
            step,
            step_1,
            location_json: RefCell::new(None),
            location_error: Mutable::new(false), 
            language: RefCell::new(None),
            language_error: Mutable::new(false), 
            persona: RefCell::new(None),
            persona_error: Mutable::new(false), 
            organization: RefCell::new(None),
            organization_error: Mutable::new(false), 
            terms: RefCell::new(false),
            terms_error: Mutable::new(None), 
            marketing: RefCell::new(false),
        }
    }

    pub fn terms_error_str(&self) -> impl Signal<Item = &'static str> {
        self.terms_error
            .signal_cloned()
            .map(|err| {
                err
                    .map(|err| err.as_str())
                    .unwrap_or("")
            })
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
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
