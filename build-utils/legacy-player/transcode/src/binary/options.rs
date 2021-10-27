use structopt::StructOpt;
use std::path::{Path, PathBuf};
use shared::config::RemoteTarget;

#[derive(Debug, StructOpt)]
#[structopt(name = "ji tap transcoder", about = "ji tap downloader/transcoder")]
pub struct Opts {
    // Soundboard Options 
    // https://jitap.net/activities/genr/play/soundboard-states 
    #[structopt(long, default_value="https://d24o39yp3ttic8.cloudfront.net/351FB160-0592-4BF6-9BFE-47E9D40EDF39/game.json")]
    pub game_json_url: String,
    // David test 002
    // #[structopt(long, default_value="https://d24o39yp3ttic8.cloudfront.net/5D00A147-73B7-43FF-A215-A38CB84CEBCD/game.json")]
    // pub game_json_url: String,
    
    // Corinne Houdini states
    // #[structopt(long, default_value="https://d24o39yp3ttic8.cloudfront.net/42C980D6-9FCE-4552-A5F2-ECFC0EA8D129/game.json")]
    // pub game_json_url: String,

    // say something options
    // https://jitap.net/activities/gen8/play/say-something-options
    // #[structopt(long, default_value="https://d24o39yp3ttic8.cloudfront.net/86DCDC1D-64CB-4198-A866-257E213F0405/game.json")]
    // pub game_json_url: String,

    /////////////////////////////////////
    #[structopt(long, default_value="D:\\Dropbox (Jewish Interactive)\\ji-cloud-media\\legacy\\examples", parse(from_os_str))]
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