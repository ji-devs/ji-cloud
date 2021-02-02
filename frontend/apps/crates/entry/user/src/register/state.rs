use futures_signals::signal::{Mutable, Signal, SignalExt};
use cfg_if::cfg_if;

pub struct State {
    pub step: Mutable<Step>
}

impl State {
    fn default() -> Self {
        Self {
            step: Mutable::new(Step::Start),
        }
    }

    cfg_if! {
        if #[cfg(all(feature = "local"))] {
            pub fn new() -> Self {
                Self::default()
                //Self::debug_step_3()
            }

            fn debug_step_1() -> Self {
                Self {
                    step: Mutable::new(Step::One(StartData::debug()))
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
            pub fn new() -> Self {
                Self::default()
            }
        }
    }
}


#[derive(Clone, Debug)]
pub enum Step {
    Start,
    One(StartData),
    Two(Step1Data),
    Three(Step2Data),
}

#[derive(Clone, Debug)]
pub struct StartData {
    pub token: String,
    pub email: String,
    pub email_verified: bool,
}

impl StartData {
    fn debug() -> Self {
        Self {
            email: "foo@example.com".to_string(),
            token: "blah".to_string(),
            email_verified: true
        }
    }
}


#[derive(Clone, Debug)]
pub struct Step1Data {
    pub start: StartData,
    pub firstname: String,
    pub username: String,
    pub lastname: String,
}

impl Step1Data {
    fn debug() -> Self {
        Self {
            start: StartData::debug(),
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
    fn debug() -> Self {
        Self {
            step_1: Step1Data::debug(),
            location_json: None,
            language: "english".to_string(),
            marketing: false,
        }
    }
}

