use serde::{Serialize, Deserialize};
use serde_repr::*;
pub use super::path::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Activity {
    Questions(Questions)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Questions {
    pub questions: Vec<Question> 
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Question {
    pub audio: String,
    pub path: Vec<PathPoint>,
}
