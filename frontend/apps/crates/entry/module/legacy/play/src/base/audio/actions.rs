use std::rc::Rc;

use components::audio::mixer::{AudioMixer, AudioHandle, AUDIO_MIXER, AudioSource, AudioSourceExt};
use gloo_timers::callback::Timeout;
use super::*;

impl AudioManager {
    pub fn play_clip(&self, url:String) {
        *self.clip.borrow_mut() = Some(AUDIO_MIXER.with(|mixer| {
            mixer.play(AudioSource::Url(url), false)
        }));
    }

    pub fn stop_clip(&self) {
        *self.clip.borrow_mut() = None; 
    }

    pub fn play_clip_on_ended(&self, url:String, f: impl Fn() + 'static) {
        *self.clip.borrow_mut() = Some(AUDIO_MIXER.with(|mixer| {
            mixer.play_on_ended(AudioSource::Url(url), false, f)
        }));
    }

    // dirty hack to win race conditions
    pub fn play_clip_next_tick(&self, url:String) {
        let clip = self.clip.clone();

        Timeout::new(0, move || { 
            *clip.borrow_mut() = Some(AUDIO_MIXER.with(|mixer| {
                mixer.play(AudioSource::Url(url), false)
            }));
        }).forget();
    }

    pub fn play_bg(&self, url:String) {
        *self.bg.borrow_mut() = Some(AUDIO_MIXER.with(|mixer| {
            mixer.play(AudioSource::Url(url), true)
        }));
    }
}