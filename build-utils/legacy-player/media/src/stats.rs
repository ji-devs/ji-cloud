use std::sync::atomic::{AtomicBool, Ordering, AtomicUsize};

pub struct Stats {
    _json_completed: AtomicBool,
    _json_count: AtomicUsize,
    _media_completed: AtomicBool,
    _media_count: AtomicUsize,
}

impl Stats {
    pub fn new() -> Self {
        Self {
            _json_completed: AtomicBool::new(false),
            _json_count: AtomicUsize::new(0),
            _media_completed: AtomicBool::new(false),
            _media_count: AtomicUsize::new(0),
        }
    }

    pub fn reset(&self) {
        self._json_completed.store(false, Ordering::SeqCst);
        self._json_count.store(0, Ordering::SeqCst);
        self._media_completed.store(false, Ordering::SeqCst);
        self._media_count.store(0, Ordering::SeqCst);
    }

    pub fn json_count(&self) -> usize {
        self._json_count.load(Ordering::SeqCst)
    }
    pub fn json_increase(&self) {
        let _ = self._json_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn json_completed(&self) -> bool {
        self._json_completed.load(Ordering::SeqCst)
    }
    pub fn json_set_completed(&self) {
        self._json_completed.store(true, Ordering::SeqCst);
    }

    pub fn media_count(&self) -> usize {
        self._media_count.load(Ordering::SeqCst)
    }
    pub fn media_increase(&self) {
        let _ = self._media_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn media_completed(&self) -> bool {
        self._media_completed.load(Ordering::SeqCst)
    }
    pub fn media_set_completed(&self) {
        self._media_completed.store(true, Ordering::SeqCst);
    }
}