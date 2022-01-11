use cfg_if::cfg_if;
use futures_signals::signal::Mutable;

use shared::domain::session::OAuthUserProfile;

cfg_if! { if #[cfg(all(feature = "local"))] {
use crate::debug::DebugRegisterStep;
}}

pub struct State {
    pub step: Mutable<Step>,
}

impl State {
    cfg_if! {
        if #[cfg(all(feature = "local"))] {
            pub fn new(step:Option<Step>) -> Self {
                let step = match crate::debug::settings().register_step {
                    None => {
                        step.unwrap_or(Step::Start)
                    },
                    Some(debug_step) => {
                        match debug_step {
                            DebugRegisterStep::Start => Step::Start,
                            DebugRegisterStep::One => Step::One(None),
                            DebugRegisterStep::Two => Step::Two(Step1Data::debug()),
                            DebugRegisterStep::Three => Step::Three(Step2Data::debug()),

                        }
                    }
                };

                Self { step: Mutable::new(step), }
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
    One(Option<OAuthUserProfile>),
    Two(Step1Data),
    Three(Step2Data),
}

#[derive(Clone, Debug)]
pub struct Step1Data {
    pub firstname: String,
    pub username: String,
    pub lastname: String,
    pub oauth_profile: Option<OAuthUserProfile>,
}

cfg_if! { if #[cfg(all(feature = "local"))] {
impl Step1Data {
    fn debug() -> Self {
        Self {
            username: "first".to_string(),
            firstname: "user".to_string(),
            lastname: "last".to_string(),
            oauth_profile: None,
        }
    }
}
}}

#[derive(Clone, Debug)]
pub struct Step2Data {
    pub step_1: Step1Data,
    pub location_json: Option<String>,
    pub language: String,
    pub persona: Vec<String>,
    pub organization: Option<String>,
    pub marketing: bool,
}

cfg_if! {if #[cfg(all(feature = "local"))] {
impl Step2Data {
    pub fn debug() -> Self {
        Self {
            step_1: Step1Data::debug(),
            location_json: None,
            language: "english".to_string(),
            persona: vec!["Teacher".to_string()],
            organization: None,
            marketing: false,
        }
    }
}
}}
