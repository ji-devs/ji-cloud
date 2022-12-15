use std::fs::File;
use std::path::Path;
use serde::{Serialize, Deserialize};
use super::context::Context;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    #[serde(rename="Jig ID")]
    pub jig_id: String,
    #[serde(rename="New Jig")]
    pub jig_new: String,
    #[serde(rename="Album")]
    pub album: String,
    #[serde(rename="ID in Album Store")]
    pub album_store_id: String,
    #[serde(rename="Author ID")]
    pub author_id: String,
    #[serde(rename="Play count")]
    pub play_count: String,
    #[serde(rename="ID for David")]
    pub game_id: String,
    #[serde(rename="Comment")]
    pub comment: String,
}

impl Record {
    pub fn load_csv(path: impl AsRef<Path>) -> Vec<Self> {
        let res:Vec<Self> = csv::Reader::from_reader(File::open(path).unwrap())
            .deserialize()
            .into_iter()
            .map(|x| x.unwrap())
            .collect();

        // sanity check
        {
            let mut game_ids = std::collections::HashSet::new();
            for record in res.iter() {
                if game_ids.contains(&record.game_id) {
                    panic!("game_id {} already exists", record.game_id);
                }
                game_ids.insert(record.game_id.clone());
            }
        }

        res
    }
}