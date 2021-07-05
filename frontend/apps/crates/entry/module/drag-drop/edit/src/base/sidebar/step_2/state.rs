use crate::base::state::Base;
use std::rc::Rc;
use dominator_helpers::signals::{RcSignalFn, rc_signal_fn};
use futures_signals::{
    map_ref, 
    signal::{Mutable, Signal, SignalExt}
};
use dominator::clone;
use components::{
    image::search::{
        state::{State as ImageSearchState, ImageSearchOptions},
        callbacks::Callbacks as ImageSearchCallbacks
    },
    audio_input::{
        options::AudioInputOptions,
        state::State as AudioInputState,
        callbacks::Callbacks as AudioCallbacks,
    },
    stickers::state::Stickers,
};
use shared::domain::jig::module::body::{Image, Audio};
use std::pin::Pin;

pub struct Step2 {
    pub base: Rc<Base>,
    pub tab: Mutable<Tab>,
}


impl Step2 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {
        let kind = match crate::debug::settings().step_2_tab {
            Some(kind) => kind,
            None => TabKind::Text
        };
        
        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
        })
    }

    pub fn drag_sticker_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.base.drag_stickers.selected_index.signal_cloned()
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Image,
    Audio,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
            Self::Audio => "audio",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Text, // uses top-level state since it must be toggled from main too
    Image(Rc<ImageSearchState>),
    Audio(RcSignalFn<Option<Rc<AudioInputState>>>),
}

impl Tab {

    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Text=> {
                Self::Text
            },
            TabKind::Image=> {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true, 
                    filters: true, 
                };

                let callbacks = ImageSearchCallbacks::new(
                    Some(clone!(base => move |image| {
                        Stickers::add_sprite(base.stickers.clone(), image);
                    }))
                );
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            },
            TabKind::Audio => {
              
                let cb = clone!(base => move || {
                    base.drag_stickers.selected_index
                        .signal()
                        .map(clone!(base => move |index| {
                            index.map(|index| {
                                let opts = AudioInputOptions::new(
                                    Some(base.drags_meta.lock_ref()[index].audio.signal_cloned())
                                );


                                let callbacks = AudioCallbacks::new(
                                    Some(clone!(base, index => move |audio:Audio| {
                                        base.set_drags_meta_audio(index, Some(audio));
                                    })),
                                    Some(clone!(base, index => move || {
                                        base.set_drags_meta_audio(index, None);
                                    })),
                                );

                                AudioInputState::new(opts, callbacks)
                            })
                        }))
                });

                Self::Audio(rc_signal_fn(cb))

            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text => TabKind::Text,
            Self::Image(_) => TabKind::Image,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

