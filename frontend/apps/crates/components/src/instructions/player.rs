use dominator::{Dom, html, clone};
use dominator::animation::MutableAnimation;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;
use shared::domain::jig::module::body::Instructions;
use crate::animation::fade::*;
use std::cell::RefCell;
use web_sys::AudioContext;
use crate::audio_mixer::{AudioMixer, AudioInstance};

pub struct InstructionsPlayer {
    data: Instructions,
    fade: Fade,
    audio: RefCell<Option<Rc<AudioInstance>>>
}

impl InstructionsPlayer {
    pub fn new(data:Instructions) -> Self {
        /*
        let data = Instructions {
            text: Some("instructions here!".to_string()),
            audio: None
        };
        */
        let animation = MutableAnimation::new(1000.0);
        Self {
            data,
            fade: Fade::new(
                FadeKind::Out,
                1000.0,
                true,
                Some(3000.0),
                None::<fn()>
            ),
            audio: RefCell::new(None),
        }
    }

    pub fn render(&self, mixer:&AudioMixer) -> Dom {
        *self.audio.borrow_mut() = self.data.audio.as_ref().map(|audio| {
            mixer.play_oneshot(audio.clone())
        });
        html!("empty-fragment", {
            .apply_if(self.data.text.is_some(), |dom| {
                let text = self.data.text.as_ref().unwrap_ji();

                self.fade.render(dom.child(
                    html!("instructions-banner", {
                        .text(text)
                    })
                ))
                
            })
        })
    }

}
