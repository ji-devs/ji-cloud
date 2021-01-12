use serde::{Serialize, Deserialize};
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
    pub fn new(id: String, name: String) -> Self {
        Self {id, name, list: Vec::new() }
    }

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

#[derive(Copy, Clone, Serialize, Debug)]
pub enum AlbumItemKind {
    Sticker,
    Background,
    Animation,
    Foreground
}

impl AlbumItemKind {
    pub fn to_str(&self) -> &'static str{
        match self {
            Self::Sticker => "sticker",
            Self::Background => "background",
            Self::Animation => "animation",
            Self::Foreground => "foreground"
        }
    }
}
impl From<u32> for AlbumItemKind {
    fn from(val:u32) -> Self {
        match val {
            0 => Self::Sticker,
            1 => Self::Background,
            2 => Self::Animation,
            3 => Self::Foreground,
            _ => panic!("unsupported item kind {}", val)
        }
    }
}



#[derive(Serialize, Debug)]
pub struct UploadAlbum {
    pub id: String,
    pub name: String,
    pub list: Vec<UploadAlbumItem>
}

impl UploadAlbum {
    pub fn new(id: String, name: String) -> Self {
        Self {id, name, list: Vec::new() }
    }
}

#[derive(Serialize, Debug)]
pub struct UploadAlbumItem {
    pub name: String,
    pub total_index: usize,
    pub album_index: usize,
    pub item_index: usize,
    pub kind: AlbumItemKind
}


impl UploadAlbumItem {
    pub fn new(total_index:usize, album_index:usize, item_index: usize, item:AlbumItem) -> Self {
        Self {
            name: item.sprite,
            total_index,
            album_index,
            item_index,
            kind: item.item_type.into()
        }
    }
}
