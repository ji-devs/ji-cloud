use dominator::animation::MutableAnimation;

use crate::animation::fade::*;
use dominator::clone;
use shared::domain::jig::module::body::Instructions;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

use crate::audio::mixer::AudioHandle;

pub struct InstructionsPlayer {
    pub(super) data: Instructions,
    pub(super) fade: Fade,
    pub(super) audio: RefCell<Option<AudioHandle>>,
    /// Will only fire when both audio and fade have ended
    pub(super) on_ended: Option<Box<dyn Fn()>>,
    pub(super) fade_ended: AtomicBool,
    pub(super) audio_ended: AtomicBool,
}

impl Drop for InstructionsPlayer {
    fn drop(&mut self) {
        log::info!("Instructions player dropped...");
    }
}

impl InstructionsPlayer {
    /// on_ended will only fire when both audio and fade have ended
    /// both audio and text are each flagged as ended if they are None
    pub fn new(data: Instructions, on_ended: Option<impl Fn() + 'static>) -> Rc<Self> {
        /*
        let data = Instructions {
            text: Some("instructions here!".to_string()),
            audio: None
        };
        */
        log::info!("Instructions Player created...");

        let _animation = MutableAnimation::new(1000.0);
        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let _self = Rc::new(Self {
            data,
            fade: Fade::new(
                FadeKind::Out,
                1000.0,
                true,
                Some(3000.0),
                Some(clone!(_self_ref => move || {
                    if let Some(_self) = _self_ref.borrow().as_ref() {
                        _self.on_fade_ended();
                    }
                })),
            ),
            audio: RefCell::new(None),
            on_ended: on_ended.map(|f| Box::new(f) as _),
            fade_ended: AtomicBool::new(false),
            audio_ended: AtomicBool::new(false),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }
}
