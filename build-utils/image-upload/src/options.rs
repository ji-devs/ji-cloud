use structopt::StructOpt;
use std::path::{Path, PathBuf};
use config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "database migrations", about = "A little util to run database migrations")]
pub struct Opts {
    // local, sandbox, or release 
    #[structopt(short, long)]
    pub remote_target: String,

    // show output 
    #[structopt(short, long)]
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
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,
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

    pub fn get_image_path(&self, album_id:&str, file_name:&str) -> PathBuf {
        self.get_creation_packs_path() 
            .join("packs")
            .join(album_id)
            .join(file_name)
    }

    pub fn get_album_path(&self, id:&str) -> PathBuf {
        self.get_creation_packs_path() 
            .join("packs")
            .join(&format!("{}.json", id))
    }

    pub fn get_manifest_path(&self) -> PathBuf {
        self.get_creation_packs_path() 
            .join("manifest.json")
    }

    pub fn get_creation_packs_path(&self) -> PathBuf {
        Path::new("D:\\JEWISH INTERACTIVE\\ji-tap-creation-packs")
            .to_path_buf()

    }

}


