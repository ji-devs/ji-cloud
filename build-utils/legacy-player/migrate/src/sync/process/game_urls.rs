use std::{future::Future, path::PathBuf};
use std::sync::Arc;
use dotenv::dotenv;
use simplelog::*;
use structopt::StructOpt;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use serde::Deserialize;
use shared::domain::jig::module::ModuleBody;
use image::gif::{GifDecoder, GifEncoder};
use image::{Frame, ImageDecoder, AnimationDecoder};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use std::process::Command;
use reqwest::Client; 
use futures::stream::{FuturesUnordered, StreamExt};
use crate::{context::Context, options::Opts};

pub struct GameJsonUrl {
    pub url: String,
    pub game_id: String
}
impl GameJsonUrl {
    pub fn new(url: String, game_id: String) -> Self {
        Self {
            url,
            game_id
        }
    }
}

pub fn load(ctx:&Context) -> Vec<GameJsonUrl> {

    if let Some(game_id) = ctx.opts.transcode_only_game_id.as_ref() {
        return vec![GameJsonUrl::new(format!("https://jitap.net/store/api/album/{}/structure/", game_id), game_id.to_string())];
    }

    #[derive(Deserialize)]
    struct Data {
        album: Album,
    }
    #[derive(Deserialize)]
    struct Album {
        fields: AlbumFields,
        pk: u32,
    }
    #[derive(Deserialize)]
    struct AlbumFields {
        structure: String,
    }

    let mut results = if ctx.opts.transcode_game_json_from_albums {
        let paths = fs::read_dir(&ctx.albums_dir).unwrap();
        let mut urls = Vec::new();

        for path in paths {
            let file = File::open(path.unwrap().path()).unwrap();
            let reader = BufReader::new(file);
            let data:Data = serde_json::from_reader(reader).unwrap();

            let game_id = format!("{}", data.album.pk);

            if ctx.opts.transcode_data_url {
                urls.push(GameJsonUrl::new(format!("https://jitap.net/store/api/album/{}/structure/", game_id), game_id));
            } else {
                urls.push(GameJsonUrl::new(data.album.fields.structure, game_id));
            }
        }
        urls
    } else {
        let list:&[(&str, &str)] = &[
                // 
                // Let's learn about tet - 7556
                // (pro only)
                ("https://d24o39yp3ttic8.cloudfront.net/223FCB2E-F1D9-42B1-80E9-BEF44FD513B6/game.json", "7556"),

                // animals - 16248
                ("https://d24o39yp3ttic8.cloudfront.net/6E5E7733-D22E-4315-AD26-CB06B6B6CB53/game.json", "16248"),

                // // David Test 002 (houdini) - 17736
                // https://jitap.net/activities/gemy/play/david-test-002
                ("https://d24o39yp3ttic8.cloudfront.net/5D00A147-73B7-43FF-A215-A38CB84CEBCD/game.json", "17736"),

                // // // Corinne Houdini - 17762
                // https://jitap.net/activities/geno/play/houdini-states
                ("https://d24o39yp3ttic8.cloudfront.net/58C85551-79A5-4E36-9794-3D3D8D3E0D31/game.json", "17762"),

                // // // Soundboard states - 17765
                // // // https://jitap.net/activities/genr/play/soundboard-states 
                ("https://d24o39yp3ttic8.cloudfront.net/9F5AD80D-7D86-4AB9-AB11-C942B162923E/game.json", "17765"),

                // // // say something options - 17746
                // // // https://jitap.net/activities/gen8/play/say-something-options
                ("https://d24o39yp3ttic8.cloudfront.net/86DCDC1D-64CB-4198-A866-257E213F0405/game.json", "17746"),

                // // // video for legacy - 17771 
                // // // https://jitap.net/activities/genx/play/ 
                ("https://d24o39yp3ttic8.cloudfront.net/64498594-B340-4B5C-87E0-615C6ACC7676/game.json", "17771"),

                // // // ask a question legacy player - 17792
                // // // https://jitap.net/activities/geoi/play/testing-ask-a-question-legacy-player
                ("https://d24o39yp3ttic8.cloudfront.net/236F4AC1-9B06-49EA-B580-4AE806B0A337/game.json", "17792"),

                // // puzzle - 17822
                // // https://jitap.net/activities/gepc/play/test-puzzles-for-legacy-player
                ("https://d24o39yp3ttic8.cloudfront.net/D9BB6E6A-03FE-4B39-A3CD-289059E118E9/game.json", "17822"),

                // // talk or type - 17820
                // // https://jitap.net/activities/gepa/play/test-talk-or-type-for-legacy-player
                ("https://d24o39yp3ttic8.cloudfront.net/2B7A33C0-BA81-4661-9202-4C0463AEC606/game.json", "17820"),
        ];
        
        list.iter()
            .map(|(a, b)| GameJsonUrl::new(String::from(*a), String::from(*b)))
            .collect()
    };

    results
    
}
