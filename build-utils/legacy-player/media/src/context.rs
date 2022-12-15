use structopt::StructOpt;
use simplelog::*;
use std::{path::{Path, PathBuf}, fs::{OpenOptions, File}};
use shared::config::RemoteTarget;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use crate::transcode::media::TranscodeCommand;

use super::{
    stats::Stats,
    record::Record,
    transcode::media::MediaInfo
};

pub struct Context {
    pub opts: Options,
    pub client: Client, 
    pub stats: Stats,
    pub records: Mutex<Vec<Record>>,
    pub medias: Mutex<Vec<MediaInfo>>,
    pub transcodes: Mutex<Vec<TranscodeCommand>>,
    pub missing_media_downloads_file: File
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Options {
    #[structopt(parse(try_from_str), default_value = "false")]
    pub dry_run: bool,

    #[structopt(parse(try_from_str), default_value = "false")]
    pub load_records: bool,

    #[structopt(parse(try_from_str), default_value = "true")]
    pub verbose: bool,

    #[structopt(long, default_value = "")]
    pub token: String,

    #[structopt(long, default_value = "release")]
    pub remote_target: String,

    #[structopt(long, default_value="/home/david/archive/csv/input.csv", parse(from_os_str))]
    //#[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/csv/input.csv", parse(from_os_str))]
    pub input_csv_path: PathBuf,

    #[structopt(long, default_value="/home/david/archive/legacy-cdn/transcode/games", parse(from_os_str))]
    //#[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/games", parse(from_os_str))]
    pub games_dir: PathBuf,

    #[structopt(long, default_value="/home/david/archive/media-infos.json", parse(from_os_str))]
    //#[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/media-infos.json", parse(from_os_str))]
    pub media_infos_file_path: PathBuf,

    #[structopt(long, default_value="/home/david/archive/transcode-infos.json", parse(from_os_str))]
    //#[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/transcode-infos.json", parse(from_os_str))]
    pub transcode_infos_file_path: PathBuf,

    #[structopt(long, default_value="/home/david/archive/missing-media.json", parse(from_os_str))]
    //#[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/missing-media.json", parse(from_os_str))]
    pub missing_media_downloads_file_path: PathBuf,

    #[structopt(parse(try_from_str), default_value = "false")]
    pub load_game_remote: bool,



    /////////////////// DOWNLOAD MODULES //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "10")]
    pub transcode_json_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub transcode_json_stop_limit: Option<usize>,

    /////////////////// DOWNLOAD MEDIA //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "10")]
    pub transcode_media_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub transcode_media_stop_limit: Option<usize>,
    #[structopt(parse(try_from_str), default_value = "true")]
    pub transcode_media_skip_download_exists: bool,
    #[structopt(parse(try_from_str), default_value = "false")]
    pub transcode_media_skip_convert_exists: bool,


    /////////////////// CONVERT MEDIA //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub transcode_media_convert_thread_size: usize,

    #[structopt(parse(try_from_str), default_value = "false")]
    pub transcode_media_parse_from_json_file: bool,
}

impl Options {
    pub fn get_remote_target(&self) -> RemoteTarget {

        match self.remote_target.as_ref() {  
            "local" => RemoteTarget::Local,
            "sandbox" => RemoteTarget::Sandbox,
            "release" => RemoteTarget::Release,
            _ => panic!("target must be local, sandbox, or release")
        }
    }
}

impl Context {
    pub fn new() -> Arc<Self> {
        let mut opts = Options::from_args();


        init_logger(opts.verbose);

        if opts.token.is_empty() {
            log::info!("no token set in opts, using env");
            opts.token = std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
        }

        let records = if opts.load_records {
            Record::load_csv(&opts.input_csv_path)
        } else {
            log::warn!("not loading records!");
            Vec::new()
        };

        let missing_media_downloads_file = {
            let mut file = OpenOptions::new();
            let mut file = file.write(true).create(true);

            //file.truncate(true)
            file.append(true)
                .open(&opts.missing_media_downloads_file_path)
                .unwrap()
        };
        Arc::new(Context {
            opts,
            client: Client::new(),
            stats: Stats::new(),
            records: Mutex::new(records),
            medias: Mutex::new(Vec::new()),
            transcodes: Mutex::new(Vec::new()),
            missing_media_downloads_file
        })
    }

    fn game_dir(&self, game_id: &str) -> PathBuf {
        self.opts.games_dir.join(game_id)
    }

    pub fn json_game_dir(&self, game_id: &str) -> PathBuf {
        self.game_dir(game_id).join("json")
    }
    pub fn media_game_dir(&self, game_id: &str) -> PathBuf {
        self.game_dir(game_id).join("media")
    }

    pub fn json_game_file_path(&self, game_id: &str) -> PathBuf {
        self.json_game_dir(game_id).join("game.json")
    }

    pub fn json_slides_dir(&self, game_id: &str) -> PathBuf {
        self.json_game_dir(game_id).join("slides")
    }

    pub fn json_slide_file_path(&self, game_id: &str, slide_id: &str) -> PathBuf {
        self.json_slides_dir(game_id).join(format!("{slide_id}.json"))
    }
}

fn init_logger(verbose: bool) {
    if verbose {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        ])
        .unwrap();
    }
}