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
                //Self::debug_step_2()
            }

            fn debug_step_1() -> Self {
                Self {
                    step: Mutable::new(Step::One(Step1Data{
                        email: "foo@example.com".to_string(),
                        token: "blah".to_string(),
                        email_verified: true
                    })),
                }
            }
            fn debug_step_2() -> Self {
                Self {
                    step: Mutable::new(Step::Two(Step2Data {
                        step_1: Step1Data{
                            email: "foo@example.com".to_string(),
                            token: "blah".to_string(),
                            email_verified: true
                        },
                        username: "blah".to_string(),
                        firstname: "blah".to_string(),
                        lastname: "blah".to_string(),
                    })),
                }
            }
            fn debug_step_3() -> Self {
                Self {
                    step: Mutable::new(Step::Three),
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
    One(Step1Data),
    Two(Step2Data),
    Three,
}

#[derive(Clone, Debug)]
pub struct Step1Data {
    pub token: String,
    pub email: String,
    pub email_verified: bool,
}


#[derive(Clone, Debug)]
pub struct Step2Data {
    pub step_1: Step1Data,
    pub firstname: String,
    pub username: String,
    pub lastname: String,
}
