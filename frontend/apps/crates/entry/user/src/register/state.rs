use futures_signals::signal::{Mutable, Signal, SignalExt};
use cfg_if::cfg_if;

pub struct State {
    pub step: Mutable<Step>
}

impl State {

    cfg_if! {
        if #[cfg(all(feature = "local"))] {
            pub fn new(step:Option<Step>) -> Self {
                Self { step: Mutable::new(step.unwrap_or(Step::Start)), }
                //Self::debug_step_3()
            }

            fn debug_step_1() -> Self {
                Self {
                    step: Mutable::new(Step::One)
                }
            }
            fn debug_step_2() -> Self {
                Self {
                    step: Mutable::new(Step::Two(Step1Data::debug()))
                }
            }
            fn debug_step_3() -> Self {
                Self {
                    step: Mutable::new(Step::Three(Step2Data::debug()))
                }
            }
        } else {
            pub fn new(step:Option<Step>) -> Self {
                Self {
                    step: Mutable::new(step.unwrap_or(Step::Start)),
                }
            }
        }
    }
}


#[derive(Clone, Debug)]
pub enum Step {
    Start,
    One,
    Two(Step1Data),
    Three(Step2Data),
}


#[derive(Clone, Debug)]
pub struct Step1Data {
    pub firstname: String,
    pub username: String,
    pub lastname: String,
}

impl Step1Data {
    fn debug() -> Self {
        Self {
            username: "first".to_string(),
            firstname: "user".to_string(),
            lastname: "last".to_string(),
        }
    }
}


#[derive(Clone, Debug)]
pub struct Step2Data {
    pub step_1: Step1Data,
    pub location_json: Option<String>,
    pub language: String,
    pub marketing: bool,
}

impl Step2Data {
    pub fn debug() -> Self {
        Self {
            step_1: Step1Data::debug(),
            location_json: None,
            language: "english".to_string(),
            marketing: false,
        }
    }
}

