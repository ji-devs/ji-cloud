use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "batch tags", about = "A little util to run ad-hoc batch tag stuff")]
pub struct Opts {
    // local, sandbox, or release 
    #[structopt(long, default_value = "local")]
    pub remote_target: String,

    #[structopt(long, default_value = "")]
    pub token: String,

    // show output 
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,

    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "10")]
    pub batch_size: usize,

    /// dry run 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,

    /// limit (debugging only) 
    #[structopt(long, parse(try_from_str), default_value = "2")]
    pub limit_debug: usize,

    /// sleep ms (debugging and dry-run only) 
    #[structopt(long, parse(try_from_str), default_value = "2000")]
    pub sleep_debug: u64,

    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub debug: bool,
}

impl Opts {
    pub fn sanitize(&mut self) {
        if self.debug {
            eprintln!("Since debug is true, forcing dry_run and verbose");
            self.dry_run = true;
            self.verbose = true;
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


