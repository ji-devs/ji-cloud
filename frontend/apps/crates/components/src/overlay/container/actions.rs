use super::state::{OVERLAY_MAP, KEY_COUNTER, OverlayContainer};
use std::sync::atomic::Ordering;
use dominator::Dom;
use std::rc::Rc;

impl OverlayContainer {
    pub fn insert<F, A>(f: F) -> usize
    where
        F: FnOnce(usize) -> A,
        A: Fn() -> Dom + 'static
    {
        OVERLAY_MAP.with(|m| {
            let key = KEY_COUNTER.fetch_add(1, Ordering::SeqCst);

            let dom_fn = Rc::new(f(key));

            m.lock_mut().insert_cloned(key, dom_fn);

            key
        })
    }

    pub fn remove(key: usize) {
        OVERLAY_MAP.with(|m| {
            m.lock_mut().remove(&key);
        });
    }
}