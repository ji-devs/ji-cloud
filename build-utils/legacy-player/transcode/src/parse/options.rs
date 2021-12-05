use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {
    #[structopt(long)]
    pub game_json_url: Option<String>,

    /////////////////////////////////////
    #[structopt(long, default_value="C:\\Users\\david\\Documents\\JI\\legacy-cdn\\games", parse(from_os_str))]
    pub dest_base_path: PathBuf,

    #[structopt(long, default_value="json", parse(from_os_str))]
    pub dest_json_dir: PathBuf,

    #[structopt(long, default_value="media", parse(from_os_str))]
    pub dest_media_dir: PathBuf,

    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    // show output 
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,
    
    /// download media 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub download_media: bool,

    /// write json
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub write_json: bool,

    /// transcode media
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_media: bool,

    /// skip files that already exist
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub skip_download_exists: bool,

    /// skip files that already exist
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub skip_transcode_exists: bool,

    /// don't panic if media is 404
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub allow_empty_media: bool,

    /// if the jump index is corrupt, just remove it 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub allow_bad_jump_index: bool,
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