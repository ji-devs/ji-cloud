use crate::register::state::{Step, Step2Data};
use super::state::*;
use std::rc::Rc;


pub fn submit(state: Rc<State>) {
    let mut ready = true;

    if !*state.terms.borrow() {
        state.terms_status.set(Some(TermsError::Unchecked));
        ready = false;
    } 

    if !ready {
        return;
    }

    next_step(state);
}

fn next_step(state: Rc<State>) {
    state.step.set(Step::Three(Step2Data{
        step_1: state.step_1.clone(), 
        location_json: state.location_json.borrow().clone(), 
        language: state.language.borrow().clone(), 
        marketing: state.marketing.borrow().clone(), 
    }));
}
