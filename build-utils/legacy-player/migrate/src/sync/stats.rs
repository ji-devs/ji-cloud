use serde::{Serialize, Deserialize};
use crate::context::Context;
pub use std::{
    path::{Path, PathBuf},
    fs::{OpenOptions, File},
    io::{SeekFrom,prelude::*},
    ops::{Deref, DerefMut}
};

pub struct Stats {
    path: PathBuf,
    data: StatsData,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StatsData {
    pub downloaded_tt_albums: bool,
    pub downloaded_jigs: bool,
    pub n_tt_albums: usize,
    pub n_jigs: usize,
    pub n_modules: usize,
}

impl StatsData {
    pub fn flush(&self, path: &PathBuf) {
        if let Ok(mut file) = File::create(&path) {
            serde_json::to_writer_pretty(file, &self).unwrap();
        } else {
            panic!("unable to create stats file at {}", path.display().to_string());
        }
    }
}

impl Stats {
    pub fn load(path: PathBuf, mut clear: bool) -> Self {

        let data = if !path.exists() || clear {
            let data = StatsData::default();
            data.flush(&path);
            data
        } else {
            let data:StatsData = serde_json::from_reader(&File::open(&path).unwrap()).unwrap();
            data
        };

        log::info!("{:#?}", data);

        Self {
            path,
            data
        }
    }

    pub fn write(&mut self) {
        self.data.flush(&self.path);
    }
}

impl Deref for Stats {
    type Target = StatsData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Stats {
    fn deref_mut(&mut self) -> &mut Self::Target { 
        &mut self.data
    }
}
