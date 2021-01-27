use legacy::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt}
};
use std::rc::Rc;
use utils::resize::{get_resize_info, ResizeInfo};

pub struct State {
    pub data: Questions,
    pub current_question: Mutable<Option<usize>>,
    pub hotspot_reveal: Mutable<Option<usize>>,
}

/* This never happens
 * Because hotspot_signal() depends on get_resize_info()
 * Which is essentially 'static
 * And that signal holds references to state
 * Creating a permanent cyclical dependency
 */
impl Drop for State {
    fn drop(&mut self) {
        log::info!("State dropped!");
    }
}

impl State {
    pub fn new(data: Questions) -> Self {
        log::info!("questions ready!");
        Self {
            data,
            current_question: Mutable::new(None),
            hotspot_reveal: Mutable::new(None),
        }

    }

    pub fn hotspot_signal(&self) -> impl Signal<Item = Option<(usize, ResizeInfo)>> {
        map_ref! {
            let index = self.hotspot_reveal.signal(),
            let info = get_resize_info().signal_cloned()
            => {
                index.map(|index| (index, info.clone()))
            }
        }
    }

    pub fn init_question(&self, index:usize) {
        self.hotspot_reveal.set(Some(index));
    }
    pub fn get_question_path(&self, index:usize) -> &[PathPoint] {
        &self.data.questions[index].path
    }

    pub fn handle_click(&self, x: i32, y: i32) {
    }
}
