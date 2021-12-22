use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {
    /////////////////////////////////////
    /// 
    /// single game id
    #[structopt(long)]
    pub game_id: Option<String>,

    // if game_id isn't supplied, loads from this url (easily generated via `ls -A1 > list.txt`)
    #[structopt(long, default_value="https://storage.googleapis.com/ji-cloud-legacy-eu-001/full-list.txt")]
    pub game_ids_list_url: String,

    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub skip_errors_log: bool,
    // if skip_errors_log, loads the errors log and skips the game ids
    #[structopt(long, default_value="https://storage.googleapis.com/ji-cloud-legacy-eu-001/logs-albums-first-2800/errors.txt")]
    pub skip_errors_log_url: String,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub skip_info_log: bool,
    // if skip_info_log, loads the info log and skips the game ids
    #[structopt(long, default_value="https://storage.googleapis.com/ji-cloud-legacy-eu-001/logs-albums-first-2800/info-release.txt")]
    pub skip_info_log_url: String,
    
    #[structopt(long, default_value="/home/david/archive/info.txt", parse(from_os_str))]
    pub info_log: PathBuf,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub clear_log_files: bool,

    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub batch_size: usize,

    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    // show output 
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
 
    // local, sandbox, or release 
    #[structopt(long, default_value = "release")]
    pub remote_target: String,

    #[structopt(long, default_value = "")]
    pub token: String,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub publish: bool,
}

impl Opts {
    pub fn sanitize(&mut self) {

        log::warn!("setting manual game id");
        //self.game_id = Some("17822".to_string());
        //self.game_id = Some("17736".to_string());
        if self.debug {
            log::warn!("sanitization: forcing dry_run since debug is true");
            self.dry_run = true;
            //self.remote_target = "local".to_string();
        } 
    }

    pub fn get_remote_target(&self) -> RemoteTarget {

        match self.remote_target.as_ref() {  
            "local" => RemoteTarget::Local,
            "sandbox" => RemoteTarget::Sandbox,
            "release" => RemoteTarget::Release,
            _ => panic!("target must be local, sandbox, or release")
        }
    }
}