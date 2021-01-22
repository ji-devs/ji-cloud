use dominator::{Dom, html, clone, with_node};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use super::{state::*, actions};
use web_sys::HtmlInputElement;
use utils::{events, routes::*};
use crate::register::state::{Step, Step2Data};

const STR_SUBMIT:&'static str = "Submit";
const STR_AGE_LABEL:&'static str = "Which age group are you interested in?";
const STR_AFFILIATION_LABEL:&'static str = "Content from which streams of Judaism do you want to see?";

pub struct Step3Page {
}

impl Step3Page {
    pub fn render(step: Mutable<Step>, step_2: Step2Data) -> Dom {
        let state = Rc::new(State::new(step, step_2));
        html!("page-register-step3", {
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "submit")
                    .property("color", "red")
                    .property("size", "medium")
                    .text(STR_SUBMIT)
                }),
            ])
        })
    }
}

