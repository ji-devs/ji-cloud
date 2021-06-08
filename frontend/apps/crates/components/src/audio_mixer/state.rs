use web_sys::AudioContext;
use std::rc::Rc;
use shared::domain::jig::Jig;

#[derive(Clone)]
pub struct AudioMixer {
    pub ctx: AudioContext
}

impl AudioMixer {
    pub fn new(ctx: AudioContext, jig: &Jig) -> Self {

        //TODO - populate jig-level effects
        Self {
            ctx
        }
    }
    pub fn new_without_jig(ctx: AudioContext) -> Self {

        Self {
            ctx
        }
    }
}
