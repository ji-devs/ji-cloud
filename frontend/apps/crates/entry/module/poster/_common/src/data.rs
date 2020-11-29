use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Poster {
}

impl Poster {
    pub async fn load(jig_id:String, module_id:String) -> Self {
        //TODO - load
        Self {} 
    }
}
