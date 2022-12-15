use serde::{de, Deserializer, Deserialize};
use serde_repr::*;
use std::{
    path::{Path, PathBuf},
    fs::File,
    io::{Write, BufReader},
    fmt
};
use super::{
    slide::*,
    activity::*,
    layer::*,
    shape::*
};
use crate::context::Context;

#[derive(Deserialize, Debug)]
pub struct SrcManifestData {
    pub data: SrcManifest,
}

pub async fn load_manifest(ctx: &Context, game_id:&str) -> SrcManifest {
    let text = if ctx.opts.load_game_remote {
        let url = format!("https://jitap.net/store/api/album/{}/structure/", game_id);
        //let url = format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/games/{}/json/game.json", game_id);

        log::info!("loading game {game_id} from {}", url);

        let text = ctx.client
            .get(&url)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .text()
            .await
            .unwrap();


        if !ctx.opts.dry_run {
            std::fs::create_dir_all(&ctx.json_game_dir(game_id));
            let path = ctx.json_game_file_path(&game_id);
            let mut file = File::create(&path).unwrap();
            write!(file, "{}", text).unwrap();
        }

        text
    } else {
        log::info!("loading game {game_id} from disk");
        let path = ctx.json_game_file_path(game_id);
        std::fs::read_to_string(path).unwrap()
    };

    // resolve some quirks
    let text = text
        .replace(r#""path": {}"#, r#""path": []"#)
        .replace(r#""originTransform": "null""#, r#""originTransform": [1,0,0,1,0,0]"#)
        .replace(r#""transform": "null""#, r#""transform": [1,0,0,1,0,0]"#);

    let api_response:SrcManifestData = match serde_json::from_str(&text) {
        Ok(data) => data,
        Err(err) => {
            panic!("for game id {game_id} error: {:?}", err);
        }
    };
    
    let manifest = api_response.data;

    if manifest.game_id() != game_id {
        panic!("wrong game_id {} != {}", manifest.game_id(), game_id);
    }

    manifest
}

#[derive(Deserialize, Debug, Clone)]
pub struct SrcManifest {
    /// Base url of the amazon bucket
    pub base_url: String,

    pub structure: ManifestStructure,

    pub album_store: AlbumStore
}


impl SrcManifest {
    pub fn game_id(&self) -> String {
        format!("{}", self.album_store.album.key)
    }

    pub fn lang_str(&self) -> &'static str {

        let tt_lang:u32 = self.album_store.album.fields.language.unwrap_or_default();

        match tt_lang { 
            16 => "da",
            8 => "nl",
            1 | 14 | 13 | 10 | 12 => "en",
            9 => "fr",
            11 => "de",
            2 => "he",
            18 => "hu",
            19 => "it",
            7 => "pt",
            6 => "ru",
            5 => "es",
            17 => "sv",
            _ => {
                log::warn!("no lang set, picking en");
                "en"
            }
        }
    }

    pub async fn load_game_id(client: &reqwest::Client, game_id:&str) -> Self {
        let text = client
            .get(&Self::url(game_id))
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .text()
            .await
            .unwrap();

        serde_json::from_str(&text).unwrap()
    }

    pub fn url(game_id:&str) -> String {
        format!("https://jitap.net/store/api/album/{}/structure/", game_id)
        //format!("https://storage.googleapis.com/ji-cloud-legacy-eu-001/games/{}/json/game.json", game_id)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ManifestStructure {
    #[serde(rename="musicFile")]
    pub music_file: Option<String>,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    pub settings: Option<ManifestSettings>,

    #[serde(rename="shuffleType")]
    pub shuffle_type: Option<ShuffleType>,

    pub version: usize,

    pub slides: Vec<Slide>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ManifestSettings {
    #[serde(rename="quizParameters")]
    pub quiz: Option<QuizSettings>,
    #[serde(rename="DisableEditing")]
    pub disable_editing: Option<u32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuizSettings {
    #[serde(rename="activityTimeLimit")]
    pub activity_time_limit: Option<f64>,

    #[serde(rename="globalLivesLimit")]
    pub global_lives_limit: Option<f64>,

    #[serde(rename="globalTimeLimit")]
    pub global_time_limit: Option<f64>,

    #[serde(rename="quizModeEnabled")]
    pub enabled: Option<bool>,
}

pub type PrimaryKey = usize;

#[repr(u8)]
#[derive(Deserialize_repr, PartialEq, Debug, Clone)]
pub enum ShuffleType {
    None = 0,
    AllSlides = 1,
    Middle = 2 // All except first and last
}

//see: https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/matrix
pub type Transform = [f64;6];

#[derive(Deserialize, Debug, Clone)]
pub struct AlbumStore {
    pub album: Album,

    #[serde(rename="pk")]
    pub key: PrimaryKey,

    pub public: Option<bool>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Album {
    #[serde(rename="pk")]
    pub key: PrimaryKey,
    pub fields: AlbumFields,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AlbumFields {
    pub name: Option<String>,
    pub description: Option<String>,
    pub author: Option<AlbumAuthor>,
    pub hash: Option<String>,
    pub language: Option<u32>,
}


#[derive(Deserialize, Debug, Clone)]
pub struct AlbumAuthor {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
