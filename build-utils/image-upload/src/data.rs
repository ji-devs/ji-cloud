use serde::Deserialize;
use std::fs::File;
use crate::options::Opts;

#[derive(Deserialize, Debug)]
pub struct Manifest {
    pub list: Vec<ManifestItem>
}

impl Manifest {
    pub fn load(opts:&Opts) -> Self {
        let file = File::open(opts.get_manifest_path()).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}

#[derive(Deserialize, Debug)]
pub struct ManifestItem {
    pub id: String,
    pub name: String
}


#[derive(Debug)]
pub struct Album {
    pub id: String,
    pub name: String,
    pub list: Vec<AlbumItem>
}

impl Album {
    pub fn load(opts:&Opts, id:String, name: String) -> Self {
        let file = File::open(opts.get_album_path(&id)).unwrap();
        let manifest:AlbumManifest = serde_json::from_reader(file).unwrap();

        Self {
            id,
            name,
            list: manifest.list
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AlbumManifest {
    pub creation_pack_id: String,
    pub list: Vec<AlbumItem>
}

#[derive(Deserialize, Debug)]
pub struct AlbumItem {
    pub thumbnail: String,
    pub sprite: String,
    pub item_type: u32
}
