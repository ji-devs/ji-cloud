pub use super::path::*;
use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Activity {
    Questions(Questions),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Questions {
    pub questions: Vec<Question>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub audio: String,
    pub path: Vec<PathPoint>,
}
