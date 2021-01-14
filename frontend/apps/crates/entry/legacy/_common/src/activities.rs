pub use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub enum Activity {
    Questions(Questions)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Questions {
    pub questions: Vec<Question> 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub audio: String,
    pub path: Vec<PathPoint>,
}
