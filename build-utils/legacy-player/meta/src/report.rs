use super::context::Context;
use std::sync::Arc;
use std::{future::Future, path::PathBuf, sync::atomic::Ordering};
use dotenv::dotenv;
use shared::domain::module::{ModuleKind, Module};
use simplelog::*;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};
use uuid::Uuid;
use std::process::Command;
use reqwest::Client; 
use serde::{Serialize, Deserialize};
use serde_json::{Result, value::RawValue};
use futures::stream::{FuturesUnordered, StreamExt};
use futures::lock::Mutex;
use std::collections::{HashMap, HashSet};
use shared::{
    api::{
        ApiEndpoint,
        PathParts,
        endpoints,
    },
    domain::{
        jig::JigResponse,
        module::{ModuleResponse, ModuleBody}
    }
};

pub async fn run(ctx:Arc<Context>) {
    ctx.stats.reset();

    let games_in_jigs = get_games_in_jigs(&ctx);
    let games_in_albums = get_games_in_albums(&ctx);

    let games_in_jigs:Vec<String> = games_in_jigs.into_iter().map(|x| x.game_id).collect();

    let games_in_jigs_but_not_albums = get_games_in_jigs_but_not_albums(&games_in_jigs, &games_in_albums);
    let games_in_albums_but_not_jigs = get_games_in_albums_but_not_jigs(&games_in_jigs, &games_in_albums);

    println!("{} games in jigs, {} games in albums, {} games in jigs but not albums, {} games in albums but not jigs",
             games_in_jigs.len(),
             games_in_albums.len(),
             games_in_jigs_but_not_albums.len(),
             games_in_albums_but_not_jigs.len()
    );

    if(!ctx.opts.dry_run) {
        let dest_path = ctx.reports_dir.join("games_in_jigs.json");
        let mut file = File::create(&dest_path).unwrap();
        serde_json::to_writer_pretty(&file, &games_in_jigs).unwrap();

        let dest_path = ctx.reports_dir.join("games_in_albums.json");
        let mut file = File::create(&dest_path).unwrap();
        serde_json::to_writer_pretty(&file, &games_in_albums).unwrap();

        let dest_path = ctx.reports_dir.join("games_in_jigs_but_not_albums.json");
        let mut file = File::create(&dest_path).unwrap();
        serde_json::to_writer_pretty(&file, &games_in_jigs_but_not_albums).unwrap();
        
        let dest_path = ctx.reports_dir.join("games_in_albums_but_not_jigs.json");
        let mut file = File::create(&dest_path).unwrap();
        serde_json::to_writer_pretty(&file, &games_in_albums_but_not_jigs).unwrap();
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct GameJigInfo {
    pub game_id: String,
    pub jig_id: String
}
pub fn get_games_in_jigs(ctx:&Context) -> Vec<GameJigInfo> {
    let mut output = HashSet::new();

    let jig_paths = fs::read_dir(&ctx.modules_dir).unwrap();

    for jig_path in jig_paths {
        let mut game_id_for_jig = None; 
        let jig_id = jig_path.as_ref().unwrap().path().file_name().unwrap().to_string_lossy().to_string();
        if jig_id.to_lowercase() != ".ds_store" {
            let module_paths = fs::read_dir(jig_path.as_ref().unwrap().path()).unwrap();
            for module_path in module_paths {
                let game_id = game_id_from_module_file(module_path.unwrap().path());
                if let Some(game_id_for_jig) = game_id_for_jig {
                    if game_id_for_jig != game_id {
                        panic!("game id changed from {} to {} for jig {:?}", game_id_for_jig, game_id, jig_path.unwrap().path());
                    }
                }

                game_id_for_jig = Some(game_id.clone());

                output.insert(GameJigInfo { jig_id: jig_id.clone(), game_id});
            }
        }
    }

    output.iter().cloned().collect()
}

pub fn get_games_on_disk(ctx:&Context) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();

    let game_paths = fs::read_dir(&ctx.games_dir).unwrap();

    for game_path in game_paths {
        let s = game_path.unwrap().file_name().to_string_lossy().to_string();
        if s.to_lowercase() != ".ds_store" {
            output.push(s);
        }
    }

    output
}

pub fn get_games_in_albums(ctx:&Context) -> Vec<String> {
    let mut output = HashSet::new();
    let album_paths = fs::read_dir(&ctx.albums_dir).unwrap();
    for album_path in album_paths {
        let game_id = game_id_from_album_store_file(album_path.unwrap().path());
        output.insert(game_id);
    }

    output.iter().cloned().collect()
}


fn get_games_in_jigs_but_not_albums(games_in_jigs: &[String], games_in_albums: &[String]) -> Vec<String> {
    let mut output = HashSet::new();
    for game_id in games_in_jigs.iter() {
        if !games_in_albums.contains(game_id) {
            output.insert(game_id.clone());
        }
    }
    output.iter().cloned().collect()
}

fn get_games_in_albums_but_not_jigs(games_in_jigs: &[String], games_in_albums: &[String]) -> Vec<String> {
    let mut output = HashSet::new();
    for game_id in games_in_albums.iter() {
        if !games_in_jigs.contains(game_id) {
            output.insert(game_id.clone());
        }
    }
    output.iter().cloned().collect()
}

fn game_id_from_module_file(path: PathBuf) -> String {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let module:Module = serde_json::from_reader(reader).unwrap();
    match module.body {
        ModuleBody::Legacy(body) => body.game_id,
        _ => panic!("module at path {:?} is not a legacy!", path)
    }
}


fn game_id_from_album_store_file(path: PathBuf) -> String {
    #[derive(Deserialize)]
    struct AlbumStore {
        album: Album
    }
    #[derive(Deserialize)]
    struct Album {
        pk: u64
    }

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let album_store:AlbumStore = serde_json::from_reader(reader).unwrap();
    album_store.album.pk.to_string()
}
