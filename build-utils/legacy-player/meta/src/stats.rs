use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};


pub struct Stats {
    _tt_albums_completed: AtomicBool,
    _tt_albums_count: AtomicUsize,
    _tt_games_completed: AtomicBool,
    _tt_games_count: AtomicUsize,
    _jigs_completed: AtomicBool,
    _jigs_count: AtomicUsize,
    _modules_completed: AtomicBool,
    _modules_count: AtomicUsize,
    _jig_update_completed: AtomicBool,
    _jig_update_count: AtomicUsize,
    _jig_create_completed: AtomicBool,
    _jig_create_count: AtomicUsize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            _tt_albums_completed: AtomicBool::new(false),
            _tt_albums_count: AtomicUsize::new(0),
            _tt_games_completed: AtomicBool::new(false),
            _tt_games_count: AtomicUsize::new(0),
            _jigs_completed: AtomicBool::new(false),
            _jigs_count: AtomicUsize::new(0),
            _modules_completed: AtomicBool::new(false),
            _modules_count: AtomicUsize::new(0),
            _jig_update_completed: AtomicBool::new(false),
            _jig_update_count: AtomicUsize::new(0),
            _jig_create_completed: AtomicBool::new(false),
            _jig_create_count: AtomicUsize::new(0),
        }
    }

    pub fn reset(&self) {
        self._tt_albums_completed.store(false, Ordering::SeqCst);
        self._tt_albums_count.store(0, Ordering::SeqCst);
        self._tt_games_completed.store(false, Ordering::SeqCst);
        self._tt_games_count.store(0, Ordering::SeqCst);
        self._jigs_completed.store(false, Ordering::SeqCst);
        self._jigs_count.store(0, Ordering::SeqCst);
        self._modules_completed.store(false, Ordering::SeqCst);
        self._modules_count.store(0, Ordering::SeqCst);
        self._jig_update_completed.store(false, Ordering::SeqCst);
        self._jig_update_count.store(0, Ordering::SeqCst);
        self._jig_create_completed.store(false, Ordering::SeqCst);
        self._jig_create_count.store(0, Ordering::SeqCst);
    }

    pub fn tt_albums_count(&self) -> usize {
        self._tt_albums_count.load(Ordering::SeqCst)
    }

    pub fn tt_albums_completed(&self) -> bool {
        self._tt_albums_completed.load(Ordering::SeqCst)
    }

    pub fn tt_albums_increase(&self) {
        let _ = self._tt_albums_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn tt_albums_set_completed(&self) {
        self._tt_albums_completed.store(true, Ordering::SeqCst);
    }

    pub fn tt_games_count(&self) -> usize {
        self._tt_games_count.load(Ordering::SeqCst)
    }

    pub fn tt_games_completed(&self) -> bool {
        self._tt_games_completed.load(Ordering::SeqCst)
    }

    pub fn tt_games_increase(&self) {
        let _ = self._tt_games_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn tt_games_set_completed(&self) {
        self._tt_games_completed.store(true, Ordering::SeqCst);
    }


    pub fn jigs_count(&self) -> usize {
        self._jigs_count.load(Ordering::SeqCst)
    }

    pub fn jigs_completed(&self) -> bool {
        self._jigs_completed.load(Ordering::SeqCst)
    }

    pub fn jigs_increase(&self) {
        let _ = self._jigs_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn jigs_set_completed(&self) {
        self._jigs_completed.store(true, Ordering::SeqCst);
    }


    pub fn modules_count(&self) -> usize {
        self._modules_count.load(Ordering::SeqCst)
    }

    pub fn modules_completed(&self) -> bool {
        self._modules_completed.load(Ordering::SeqCst)
    }

    pub fn modules_increase(&self) {
        let _ = self._modules_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn modules_set_completed(&self) {
        self._modules_completed.store(true, Ordering::SeqCst);
    }


    pub fn jig_update_count(&self) -> usize {
        self._jig_update_count.load(Ordering::SeqCst)
    }

    pub fn jig_update_completed(&self) -> bool {
        self._jig_update_completed.load(Ordering::SeqCst)
    }

    pub fn jig_update_increase(&self) {
        let _ = self._jig_update_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn jig_update_set_completed(&self) {
        self._jig_update_completed.store(true, Ordering::SeqCst);
    }

    pub fn jig_create_count(&self) -> usize {
        self._jig_create_count.load(Ordering::SeqCst)
    }

    pub fn jig_create_completed(&self) -> bool {
        self._jig_create_completed.load(Ordering::SeqCst)
    }

    pub fn jig_create_increase(&self) {
        let _ = self._jig_create_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn jig_create_set_completed(&self) {
        self._jig_create_completed.store(true, Ordering::SeqCst);
    }
}
