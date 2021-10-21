use shared::domain::jig::{JigData, JigId, module::{ModuleId, body::{_groups::design::{Backgrounds, Sticker}, ThemeChoice, Instructions, legacy::{slide::Slide, ModuleData as RawData}}}};
use components::{audio::mixer::AudioMixer, module::_common::play::prelude::*};
use utils::prelude::*;
use web_sys::Worker;
use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};
use futures_signals::signal::Mutable;
use awsm_web::{
    loaders::fetch::fetch_url,
    workers::new_worker_from_js,
};

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig: JigData,
    pub theme_id: ThemeId,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub game_id: String,
    pub slide_id: String,
    pub slide: Slide,
    pub workers: RefCell<HashMap<WorkerKind, WorkerList>>
}

#[derive(Default)]
pub struct WorkerList {
    pub list: Vec<Worker>,
    pub curr_index: AtomicUsize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum WorkerKind {
    GifConverter
}

static GIF_CONVERTER_SRC:&str = include_str!("design/sprite/gif-converter.js");

impl WorkerKind {
    pub fn make_worker(&self) -> Worker {
        match self {
            Self::GifConverter => {
                new_worker_from_js(GIF_CONVERTER_SRC, None).unwrap_ji()
            }
        }
    } 
}

impl WorkerList {
    pub fn next(&self) -> Option<&Worker> {
        if self.list.is_empty() {
            None
        } else {
            let index = self.curr_index.load(Ordering::SeqCst);
            if index >= self.list.len() {
                self.curr_index.store(0, Ordering::SeqCst);
                self.list.get(0)
            } else {
                self.curr_index.store(index + 1, Ordering::SeqCst);
                self.list.get(index)
            }
        }
    }
}

impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, (), ()>) -> Rc<Self> {
        let InitFromRawArgs {
            jig_id,
            module_id,
            jig,
            raw,
            theme_id,
            ..
        } = init_args;

        let url = utils::path::legacy_cdn_url(format!("{}/json/slides/{}.json", raw.game_id, raw.slide_id));

        let slide:Slide = fetch_url(&url)
            .await
            .unwrap_ji()
            .json_from_str()
            .await
            .unwrap_ji();

        Rc::new(Self {
            jig_id,
            module_id,
            jig,
            theme_id,
            module_phase: init_args.play_phase,
            game_id: raw.game_id,
            slide_id: raw.slide_id,
            slide,
            workers: RefCell::new(HashMap::new())
        })
    }

    pub fn media_url<T: AsRef<str>>(&self, path:T) -> String {
        utils::path::legacy_cdn_url(&format!("{}/media/slides/{}/{}", self.game_id, self.slide_id, path.as_ref()))
    }

    pub fn workers_len(&self) -> usize {
        let workers = self.workers.borrow();
        let mut len = 0;

        for workers in workers.values() {
            len += workers.list.len();
        }

        len
    }

    pub fn get_worker(&self, kind: WorkerKind) -> Worker {
        let total_len = self.workers_len();

        let mut workers = self.workers.borrow_mut();
        let workers = workers.entry(kind).or_default();

        // try to limit it to max_workers, but if we have none for this src
        // create it anyway
        if total_len <= crate::config::MAX_WORKERS || workers.list.is_empty() {
            let worker = kind.make_worker(); 
            workers.list.push(worker.clone());
            worker 
        } else {
            workers.next().unwrap_ji().clone()
        }
    }
}

impl BaseExt for Base {
    fn get_instructions(&self) -> Option<Instructions> {
        None
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
