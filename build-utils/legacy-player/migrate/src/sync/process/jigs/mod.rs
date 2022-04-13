pub mod download;
pub mod update;
pub mod create;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use futures::lock::Mutex;
use shared::domain::jig::JigResponse;
use std::sync::Arc;

#[derive(Default, Serialize, Deserialize)]
pub struct JigsLookup {
    pub game_to_jig: HashMap<String, Vec<JigResponse>>,
    pub jig_to_game: HashMap<String, Vec<String>>,
}

pub type JigsLookupArc = Arc<Mutex<JigsLookup>>;
