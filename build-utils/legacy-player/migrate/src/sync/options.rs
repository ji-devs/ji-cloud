use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {

    /////////////////// GLOBAL //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub set_debug_values: bool,
    //#[structopt(long, default_value="/Users/dakom/Downloads/output", parse(from_os_str))]
    //#[structopt(long, default_value="E:\\JI\\output", parse(from_os_str))]
    #[structopt(long, default_value="/home/david/archive/legacy-cdn/transcode", parse(from_os_str))]
    pub output_dir: PathBuf,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub dry_run: bool,
    // clear the stats before running
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub clear_stats: bool,
    /// will set debug values
    // local, sandbox, or release 
    #[structopt(long, default_value = "release")]
    pub remote_target: String,
    #[structopt(long, default_value = "")]
    pub token: String,
    #[structopt(short, long, parse(try_from_str), default_value = "true")]
    pub verbose: bool,
    // clear the log files before running
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub clear_log_files: bool,
    // if false, won't download the albums at all (will still read them as needed)
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub process_download_albums: bool,
    // if false, won't update or create jigs
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub process_update_jigs: bool,
    // if false, won't do any transcoding of json
    // however, currently, will be forced to true if process_transcode_game_media is true
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub process_transcode_game_json: bool,
    // if false, won't do any transcoding of media
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub process_transcode_game_media: bool,

    /////////////////// ALBUMS //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub albums_skip_if_stats_completed: bool,
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub albums_per_page: u32,
    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub albums_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub albums_page_stop_limit: Option<usize>,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub albums_warn_exists: bool,

    /////////////////// DOWNLOAD JIGS //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_jigs_skip_if_stats_completed: bool,
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub download_jigs_batch_size: usize,
    #[structopt(long, parse(try_from_str))]
    pub download_jigs_page_stop_limit: Option<usize>,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub download_jigs_warn_exists: bool,
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub download_jigs_warn_game_mapping_exists: bool,

    /////////////////// UPDATE JIGS //////////////////////////////////
    /// will create the jig if none exists for a given game id
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub update_jigs_create_if_not_exist: bool,
    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "0")]
    pub update_jigs_batch_size: usize,
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub update_jigs_skip_cover_page: bool,

    /////////////////// TRANSCODE ALL //////////////////////////////////
    #[structopt(long, parse(try_from_str))]
    pub transcode_only_game_id: Option<String>,
    /////////////////// TRANSCODE JSON //////////////////////////////////
    /// batch size to help throttle connections 
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub transcode_json_batch_size: usize,
    ///filter out all loaded albums for just a specific game id
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_game_json_from_albums: bool,
    /// skip transcoding json if it was previously downloaded
    /// setting to false will re-download all jsons (that are not in a module)
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub transcode_game_json_skip_json_exists: bool,
    /// Attempt to load the game json locally from file before remotely
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_game_json_file_first: bool,


    /////////////////// TRANSCODE MEDIA //////////////////////////////////
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_download_media: bool,
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_convert_media: bool,
    #[structopt(long, parse(try_from_str), default_value = "100")]
    pub transcode_media_download_batch_size: usize,
    #[structopt(long, parse(try_from_str), default_value = "4")]
    pub transcode_media_convert_thread_size: usize,
    //if use the data_url via API instead of structure url in album 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_data_url: bool,
    /// if this is set, will use game json from albums folder
    /// if game_json_url isn't set 
    /// otherwise, uses hardcoded local vec
    /// don't panic if media is 404
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_allow_empty_media: bool,

    // another gate for panicking *after* the allow_empty_media check 
    // useful for just gathering more info into logs without aborting
    // or in other words for knowing which games to skip, ultimately, without stopping the process altogether
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub transcode_panic_on_404_error: bool,

    /// if the jump index is corrupt, just remove it 
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_allow_bad_jump_index: bool,
    #[structopt(long, parse(try_from_str), default_value = "false")]
    pub transcode_panic_on_manifest_parse_error: bool,

    /// skip files that already exist
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_media_skip_download_exists: bool,

    /// skip files that already exist
    #[structopt(long, parse(try_from_str), default_value = "true")]
    pub transcode_media_skip_convert_exists: bool,


}

impl Opts {
    pub fn sanitize(&mut self) {
        if self.set_debug_values {
            self.albums_page_stop_limit = Some(3);
            self.download_jigs_page_stop_limit = Some(3);
            //self.transcode_only_game_id = Some("18514".to_string());
            //self.transcode_only_game_id = Some("37".to_string());
            self.transcode_only_game_id = Some("12438".to_string());
        } 

        if self.token.is_empty() {
            log::info!("no token set in opts, using env");
            self.token = std::env::var("LOCAL_API_AUTH_OVERRIDE").expect("Need LOCAL_API_AUTH_OVERRIDE in .env")
        }

        if self.process_transcode_game_media {
            self.process_transcode_game_json = true;
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


