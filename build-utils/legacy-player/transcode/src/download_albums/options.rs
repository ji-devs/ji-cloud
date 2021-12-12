use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {
    /////////////////////////////////////
    #[structopt(long, default_value="C:\\Users\\david\\Documents\\JI\\legacy-cdn\\albums", parse(from_os_str))]
    pub dest_dir: PathBuf,

    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub per_page: u32,

    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    // show output 
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
}

impl Opts {
    pub fn sanitize(&mut self) {

        if self.debug {
            log::warn!("sanitization: forcing dry_run since debug is true");
            self.dry_run = true;
            //self.remote_target = "local".to_string();
        } 
    }
}