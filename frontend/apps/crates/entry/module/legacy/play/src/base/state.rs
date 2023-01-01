use super::{
    actions::{StageClick, StageClickContinuation},
    audio::AudioManager,
    design::sticker::animation::WorkerKind,
};
use awsm_web::loaders::fetch::fetch_url;
use components::module::_common::play::prelude::*;
use futures_signals::signal::Mutable;
use shared::domain::module::body::legacy::activity::Activity;
use shared::domain::{
    asset::{Asset, AssetId},
    module::{
        body::{
            legacy::{slide::Slide, ModuleData as RawData},
            Instructions,
        },
        ModuleId,
    },
};
use std::collections::VecDeque;
use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
use utils::prelude::*;
use web_sys::Worker;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub asset: Asset,
    pub theme_id: ThemeId,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub game_id: String,
    pub slide_id: String,
    pub slide: Slide,
    pub workers: RefCell<HashMap<WorkerKind, WorkerList>>,
    pub bg_click_listener: RefCell<Option<Box<dyn FnMut()>>>,
    pub start_listeners: RefCell<Vec<Box<dyn FnMut()>>>,
    pub stage_click_listeners:
        RefCell<VecDeque<Box<dyn FnMut(StageClick) -> StageClickContinuation>>>,
    pub audio_manager: AudioManager,
    pub stage_click_allowed: AtomicBool,
    pub has_started: AtomicBool,
    pub has_navigated: AtomicBool,
}

#[derive(Default)]
pub struct WorkerList {
    pub list: Vec<Worker>,
    pub curr_index: AtomicUsize,
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
            asset_id,
            module_id,
            asset,
            raw,
            theme_id,
            ..
        } = init_args;

        //log::info!("{:#?}", raw);

        // updated in latest migration tool
        // but not all previous modules
        let slide_id = raw.slide_id.trim_matches('/').replace('/', "-").to_string();

        let url =
            utils::path::legacy_cdn_url(format!("{}/json/slides/{}.json", raw.game_id, slide_id));

        log::info!("loading {}", url);

        let slide: Slide = fetch_url(&url)
            .await
            .unwrap_ji()
            .json_from_str()
            .await
            .unwrap_ji();

        let _self = Rc::new(Self {
            asset_id,
            module_id,
            asset,
            theme_id,
            module_phase: init_args.play_phase,
            game_id: raw.game_id,
            slide_id,
            slide,
            workers: RefCell::new(HashMap::new()),
            bg_click_listener: RefCell::new(None),
            start_listeners: RefCell::new(Vec::new()),
            stage_click_listeners: RefCell::new(VecDeque::new()),
            audio_manager: AudioManager::new(),
            stage_click_allowed: AtomicBool::new(false),
            has_started: AtomicBool::new(false),
            has_navigated: AtomicBool::new(false),
        });

        // TODO- set after done preloading
        _self.finished_preload();

        _self
    }

    pub fn finished_preload(&self) {
        self.module_phase.set_neq(ModulePlayPhase::Init);
    }
    pub fn set_bg_listener(&self, f: impl FnMut() + 'static) {
        *self.bg_click_listener.borrow_mut() = Some(Box::new(f));
    }

    pub fn insert_start_listener(&self, f: impl FnMut() + 'static) {
        self.start_listeners.borrow_mut().push(Box::new(f));
    }

    pub fn insert_stage_click_listener(
        &self,
        f: impl FnMut(StageClick) -> StageClickContinuation + 'static,
    ) {
        self.stage_click_listeners
            .borrow_mut()
            .push_front(Box::new(f));
    }

    pub fn activity_media_url<T: AsRef<str>>(&self, path: T) -> String {
        utils::path::legacy_cdn_url(&format!(
            "{}/media/slides/{}/activity/{}",
            self.game_id,
            self.slide_id,
            path.as_ref()
        ))
    }
    pub fn design_media_url<T: AsRef<str>>(&self, path: T) -> String {
        utils::path::legacy_cdn_url(&format!(
            "{}/media/slides/{}/{}",
            self.game_id,
            self.slide_id,
            path.as_ref()
        ))
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

    pub fn should_render_design(&self) -> bool {
        match self.slide.activity.as_ref() {
            Some(Activity::Puzzle(_)) => false,
            _ => true,
        }
    }
}

impl BaseExt for Base {
    fn play(state: Rc<Self>) {
        state.has_started.store(true, Ordering::SeqCst);
        for f in state.start_listeners.borrow_mut().iter_mut() {
            f();
        }
    }

    fn get_instructions(&self) -> Option<Instructions> {
        None
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
