use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;
use std::sync::{Arc, Mutex};
use reqwest::Client;
use crate::record::Record;
use super::stats::Stats;

pub struct Context {
    pub opts: Options,
    pub client: Client, 
    pub albums_dir: PathBuf,
    pub games_dir: PathBuf,
    pub jigs_dir: PathBuf,
    pub modules_dir: PathBuf,
    pub reports_dir: PathBuf,
    pub stats: Stats,
    pub records: Mutex<Vec<Record>>,
}

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Options {
    #[structopt(parse(try_from_str), default_value = "false")]
    pub dry_run: bool,

    #[structopt(parse(try_from_str), default_value = "true")]
    pub verbose: bool,

    #[structopt(long, default_value = "")]
    pub token: String,

    #[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/games", parse(from_os_str))]
    pub games_dir: PathBuf,

    #[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data", parse(from_os_str))]
    //#[structopt(long, default_value="E:\\JI\\output", parse(from_os_str))]
    //#[structopt(long, default_value="/home/david/archive/legacy-cdn/transcode", parse(from_os_str))]
    pub output_dir: PathBuf,

    #[structopt(long, default_value = "release")]
    pub remote_target: String,

    #[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/csv/input.csv", parse(from_os_str))]
    pub input_csv_path: PathBuf,

    #[structopt(long, default_value="/Users/dakom/Downloads/jigzi-data/csv/output.csv", parse(from_os_str))]
    pub output_csv_path: PathBuf,

    #[structopt(parse(try_from_str), default_value = "false")]
    pub load_game_remote: bool,

    /////////////////// DOWNLOAD ALBUMS //////////////////////////////////
    #[structopt(long, parse(try_from_str))]
    pub download_albums_page_stop_limit: Option<usize>,

    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub download_albums_per_page: u32,

    /////////////////// DOWNLOAD JIGS //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_jigs_skip_if_stats_completed: bool,
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub download_jigs_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub download_jigs_page_stop_limit: Option<usize>,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_jigs_warn_exists: bool,

    /////////////////// DOWNLOAD MODULES //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_modules_skip_if_stats_completed: bool,
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub download_modules_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub download_modules_jig_stop_limit: Option<usize>,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_modules_warn_exists: bool,


    /////////////////// CREATE JIGS //////////////////////////////////
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

        if opts.token.is_empty() {
            log::info!("no token set in opts, using env");
            opts.token = std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
        }

        //opts.download_albums_page_stop_limit = Some(1);
        //opts.download_jigs_page_stop_limit = Some(1);
        //opts.download_modules_jig_stop_limit = Some(1);

        let albums_dir = opts.output_dir.join("albums");
        let games_dir = opts.output_dir.join("games");
        let jigs_dir = opts.output_dir.join("jigs");
        let modules_dir = opts.output_dir.join("modules");
        let reports_dir = opts.output_dir.join("reports");

        if !opts.dry_run {
            std::fs::create_dir_all(&albums_dir);
            std::fs::create_dir_all(&games_dir);
            std::fs::create_dir_all(&jigs_dir);
            std::fs::create_dir_all(&modules_dir);
            std::fs::create_dir_all(&reports_dir);
        }

        let records = Record::load_csv(&opts.input_csv_path);

        Arc::new(Context {
            opts,
            client: Client::new(),
            albums_dir,
            games_dir,
            jigs_dir,
            modules_dir,
            reports_dir,
            records: Mutex::new(records),
            stats: Stats::new()
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
