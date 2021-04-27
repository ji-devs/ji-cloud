use dominator::{html, Dom, clone};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{SignalVec, SignalVecExt},
};
use shared::{
    domain::{
        jig::module::body::{Audio, Instructions},
        audio::AudioId,
    },
    media::MediaLibrary
};
use crate::audio_input::{
    dom::render as render_audio_input,
    options::AudioInputOptions,
    state::State as AudioState,
};
use futures::stream::StreamExt;

pub const STR_INSTRUCTIONS_LABEL:&'static str = "Written instructions";
pub const STR_INSTRUCTIONS_PLACEHOLDER:&'static str = "Type instructions";

pub struct InstructionsEditor {
    instructions: Mutable<Instructions>,
    save: Rc<Box<dyn Fn(Instructions, bool)>>, //the flag indicates whether it should be pushed to history too
}

impl InstructionsEditor {
    pub fn new(instructions: Mutable<Instructions>, save: Box<dyn Fn(Instructions, bool)>) -> Self {
        Self {
            instructions,
            save: Rc::new(save),
        }
    }

    pub fn render(&self) -> Dom {
        html!("div", {
            .children(&mut [
                self.render_text(),
                self.render_audio()
            ])
        })
    }

    fn text_signal(&self) -> impl Signal<Item = String> {
        self.instructions.signal_cloned().map(|instructions| {
            match instructions.text {
                None => "".to_string(),
                Some(text) => text
            }
        })
    }
    fn audio_signal(&self) -> impl Signal<Item = Option<Audio>> {
        self.instructions.signal_cloned().map(|instructions| {
            instructions.audio
        })
    }

    fn render_text(&self) -> Dom {
        let Self {instructions, save} = self;

        html!("input-form-textarea", {
            .property_signal("value", self.text_signal())
            .property("label", STR_INSTRUCTIONS_LABEL)
            .property("placeholder", STR_INSTRUCTIONS_PLACEHOLDER)
            .property("rows", 4)
            //Input saves every character
            //Change also pushes history
            .event(clone!(save, instructions => move |evt:events::CustomInput| {
                let mut instructions = instructions.get_cloned();
                let text = evt.value();
                if text == "" {
                    instructions.text = None;
                } else {
                    instructions.text = Some(text);
                }
                save(instructions, false);
            }))
            .event(clone!(save, instructions => move |evt:events::CustomChange| {
                let mut instructions = instructions.get_cloned();
                let text = evt.value();
                if text == "" {
                    instructions.text = None;
                } else {
                    instructions.text = Some(text);
                }
                save(instructions, true);
            }))
        })
    }
    fn render_audio(&self) -> Dom {
        let Self {instructions, save} = self;

        let opts = AudioInputOptions {
            on_change: Some(Box::new(clone!(instructions, save => move |audio_id| {
                let mut instructions = instructions.get_cloned();
                instructions.audio = audio_id.map(|id| {
                    Audio {
                        id,
                        lib: MediaLibrary::User
                    }
                });

                save(instructions, true); 
            }))),

            audio_id: self.instructions.get_cloned().audio.map(|audio| audio.id),
        };

        let audio_state = Rc::new(AudioState::new(opts)); 

        html!("empty-fragment", {
            .future(self.audio_signal()
                    .to_stream()
                    .skip(1)
                    .for_each(clone!(audio_state => move |audio| {
                        //This just happens when history is changed really
                        audio_state.set_audio_id_ext(audio.map(|audio| audio.id));
                        async {}
                    }))
            )
            .child(render_audio_input(audio_state, None))
        })
    }
}
