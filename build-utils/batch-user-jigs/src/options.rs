use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {
    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "1000")]
    pub batch_size: usize,

    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub update_screenshots: bool,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub update_background_music: bool,

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

    #[structopt(long, default_value = "v2.local.ZL54qXfXrtqWoubng3huU7rNF-u3pZPEWuR3RR7xSKQ5bylue2RMh-M_fQJ161ADFh1txRFzJQwbjLRaa4XjyXcvzCbr2CKgaliPmnapiq4mevEXjKH9531Z1NlHYjleqlQFES5c_N42CEBeyqfUOMeMNLB4r0zRadmmepghgbzEZ2OrWhW_7SMJ7wNnXPp3sHq-bD24MhuXrkgbq4okO9we7snLqGWLhyM0BEW0Y6KX0P7ksLcpFxKHPdtBDCDglIQo-GkqL5QRUGA5PJJ4AmJODAnSZ-FGT31iKNMiLyiPgSplRd-RL7g9sKwOpW54jRBndMZVzfg9aBJ0-K07omRml2MsCAHFLXwHlETvKeFd6UwXrZJhzct9wMJ7.YXV0aG9yaXplZA")]
    pub token: String,
}

impl Opts {
    pub fn sanitize(&mut self) {
        if self.debug {
            log::warn!("sanitization: forcing dry_run since debug is true");
            self.dry_run = true;
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
