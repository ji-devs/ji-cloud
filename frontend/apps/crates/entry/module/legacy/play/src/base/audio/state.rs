use components::audio::mixer::{AudioMixer, AudioHandle};
use gloo_timers::callback::Timeout;
use std::cell::RefCell;
use std::rc::Rc;
pub struct AudioManager {
    pub clip: Rc<RefCell<Option<AudioHandle>>>,
    pub bg: Rc<RefCell<Option<AudioHandle>>>,

}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            clip: Rc::new(RefCell::new(None)),
            bg: Rc::new(RefCell::new(None)),
        }
    }
}