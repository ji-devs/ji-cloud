use std::{cell::RefCell, rc::Rc};

use super::super::container::OverlayContainer;
use dominator::{Dom, clone, DomBuilder};

pub struct OverlayHandle {
    pub(super) key: usize
}

impl OverlayHandle {
    pub fn new(f: impl Fn() -> Dom + 'static) -> Rc<Self> {
        let key = OverlayContainer::insert(move |key| {
            f
        });

        Rc::new(Self { key })
    }

    pub fn lifecycle<A: Clone + 'static>(f: impl Fn() -> Dom + 'static) -> impl FnOnce(DomBuilder<A>) -> DomBuilder<A>
    {
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