use crate::base::state::*;
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
    audio::input::{
        AudioInputOptions,
        AudioInput,
        AudioInputCallbacks,
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
            None => TabKind::Select
        };
        
        let tab = Mutable::new(Tab::new(base.clone(), kind));

        Rc::new(Self {
            base,
            tab
        })
    }

}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Select,
    Audio,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Audio => "audio",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Select,
    Audio(RcSignalFn<Option<Rc<AudioInput>>>),
}

impl Tab {

    pub fn new(base: Rc<Base>, kind:TabKind) -> Self {
        match kind {
            TabKind::Select => Self::Select,
            TabKind::Audio => {
                let cb = clone!(base => move || {
                    base.selected_item_kind_signal()
                        .map(clone!(base => move |index_item_kind| {
                            index_item_kind.and_then(|(index, item_kind)| {

                                match item_kind {
                                    ItemKind::Static => None,
                                    ItemKind::Interactive(data) => {
                                        let opts = AudioInputOptions::new(
                                            Some(data.audio.signal_cloned())
                                        );

                                        let callbacks = AudioInputCallbacks::new(
                                            Some(clone!(base => move |audio:Audio| {
                                               base.set_drag_item_audio(index, Some(audio));
                                            })),
                                            Some(clone!(base => move || {
                                               base.set_drag_item_audio(index, None);
                                            })),
                                        );

                                        Some(AudioInput::new(opts, callbacks))
                                    }
                                }
                            })
                        }))
                });

                Self::Audio(rc_signal_fn(cb))

            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Select => TabKind::Select,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

