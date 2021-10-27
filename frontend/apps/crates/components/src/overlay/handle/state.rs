use std::{cell::RefCell, rc::Rc};

use super::super::container::OverlayContainer;
use dominator::{clone, Dom, DomBuilder};

pub struct OverlayHandle {
    pub(super) key: usize,
}

impl OverlayHandle {
    /// not typically used, mostly for internal purposes
    /// but still public to support different potential use-cases
    pub fn new(f: impl Fn() -> Dom + 'static) -> Rc<Self> {
        let key = OverlayContainer::insert(move |_key| f);

        Rc::new(Self { key })
    }

    /// the usual way. meant to be passed to some parent's .apply()
    /// will tie this handle to the lifecycle of the parent
    pub fn lifecycle<A: Clone + 'static>(
        f: impl Fn() -> Dom + 'static,
    ) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A> {
        let keep_alive = Rc::new(RefCell::new(Some(Self::new(f))));

        clone!(keep_alive => move |dom| {
            dom
                .after_removed(clone!(keep_alive => move |_| {
                    let _ = keep_alive.borrow_mut().take();
                }))
        })
    }
}

impl Drop for OverlayHandle {
    fn drop(&mut self) {
        log::info!("overlay {} dropped", self.key);

        OverlayContainer::remove(self.key);
    }
}
