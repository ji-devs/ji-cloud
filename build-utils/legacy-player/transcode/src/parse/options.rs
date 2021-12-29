use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {

    //if use the data_url via API instead of structure url in album 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub data_url: bool,
    /// if this is set, will use game json from albums folder
    /// if game_json_url isn't set 
    /// otherwise, uses hardcoded local vec
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub game_json_from_albums: bool,
    #[structopt(long, default_value="/home/david/archive/warnings.txt", parse(from_os_str))]
    pub warnings_log: PathBuf,
    #[structopt(long, default_value="/home/david/archive/errors.txt", parse(from_os_str))]
    pub errors_log: PathBuf,
    #[structopt(long, default_value="/home/david/archive/finished.txt", parse(from_os_str))]
    pub finished_log: PathBuf,
    /// debug mode 
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub debug: bool,

    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub keep_going_if_manifest_parse_error: bool,

    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub clear_log_files: bool,

    /// skip target directories that already exist (if using finished.txt, probably unneccessary)
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub skip_dir_exists: bool,

    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub batch_size: usize,

    #[structopt(long)]
    pub game_json_url: Option<String>,

    #[structopt(long, default_value="/home/david/archive/legacy-cdn/albums", parse(from_os_str))]
    pub game_json_albums_dir: PathBuf,

    /////////////////////////////////////
    #[structopt(long, default_value="/home/david/archive/legacy-cdn/games", parse(from_os_str))]
    pub dest_base_path: PathBuf,

    #[structopt(long, default_value="json", parse(from_os_str))]
    pub dest_json_dir: PathBuf,

    #[structopt(long, default_value="media", parse(from_os_str))]
    pub dest_media_dir: PathBuf,


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

    // another gate for panicking *after* the allow_empty_media check 
    // useful for just gathering more info into logs without aborting
    // or in other words for knowing which games to skip, ultimately, without stopping the process altogether
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub panic_on_404_error: bool,

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