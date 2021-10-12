use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "database migrations", about = "A little util to run database migrations")]
pub struct Opts {
    #[structopt(long, default_value="web-stress-test")]
    pub base_id: String,
    #[structopt(long, default_value="D:\\Dropbox (Jewish Interactive)\\ji-cloud-media\\legacy\\examples", parse(from_os_str))]
    pub src_path: PathBuf,
    #[structopt(long, default_value="game.json")]
    pub src_json: PathBuf,

    #[structopt(long, default_value="ji", parse(from_os_str))]
    pub dest_dir: PathBuf,

    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    // show output 
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,
}

impl Opts {
    pub fn sanitize(&mut self) {
        if self.debug {
            //log::warn!("sanitization: forcing dry_run since debug is true");
            //self.dry_run = true;
            //self.remote_target = "local".to_string();
        } 
    }

}


