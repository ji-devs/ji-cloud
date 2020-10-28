use crate::data::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use crate::options::Opts;

#[derive(Serialize, Debug, Default)]
pub struct Report {
    pub n_to_upload: usize,
    pub n_uploaded: usize,
    pub n_skipped: usize,
    pub albums: Vec<ReportAlbum>
}

#[derive(Serialize, Debug)]
pub struct ReportAlbum {
    pub id: String,
    pub name: String,
    pub list: Vec<ReportAlbumItem>
}

impl From<&Album> for ReportAlbum {
    fn from(album:&Album) -> Self {
        let list:Vec<ReportAlbumItem> = 
            album.list
                .iter()
                .map(|item| ReportAlbumItem {
                    name: item.sprite.clone(),
                    remote_id: None,
                    is_skipped: false,
                    kind: item.item_type.into()
                })
                .collect();

        Self {
            id: album.id.clone(),
            name: album.name.clone(),
            list
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ReportAlbumItem {
    pub name: String,
    pub remote_id: Option<String>,
    pub is_skipped: bool,
    pub kind: AlbumItemKind
}

#[derive(Serialize, Debug)]
pub struct CsvItem<'a> {
    pub album_id: &'a str,
    pub album_name: &'a str,
    pub name: &'a str,
    pub remote_id: &'a Option<String>,
    pub kind: AlbumItemKind,
    pub is_skipped: bool,
}

impl Report {
    pub fn write_csv(&self, opts:&Opts) {

        std::fs::create_dir_all(opts.report_csv_path.parent().unwrap()).unwrap();
        let file = File::create(&opts.report_csv_path).unwrap();
        let mut wtr = csv::Writer::from_writer(file);
        for album in self.albums.iter() {
            let album_id = &album.id;
            let album_name = &album.name;
            for item in album.list.iter() {
                let csv_item = CsvItem {
                    album_id,
                    album_name,
                    name: &item.name,
                    remote_id: &item.remote_id,
                    kind: item.kind,
                    is_skipped: item.is_skipped
                };
                wtr.serialize(csv_item).unwrap();
            }
        }
        wtr.flush().unwrap();
    }
}
