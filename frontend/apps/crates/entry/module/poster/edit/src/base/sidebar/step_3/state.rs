use crate::base::sidebar::state::Sidebar;
use components::audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions};
use dominator::clone;
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;

pub struct Step3 {
    pub sidebar: Rc<Sidebar>,
    pub audio: Rc<AudioInput>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let audio = {
            let base = Rc::clone(&sidebar.base);

            let opts = AudioInputOptions::new(Some(
                base.instructions
                    .signal_cloned()
                    .map(|instructions| instructions.audio),
            ));

            let callbacks = AudioInputCallbacks::new(
                Some(clone!(base => move |audio:Audio| {
                    base.set_instructions_audio(Some(audio));
                })),
                Some(clone!(base => move || {
                    base.set_instructions_audio(None);
                })),
            );

            AudioInput::new(opts, callbacks)
        };

        Rc::new(Self { sidebar, audio })
    }
}
