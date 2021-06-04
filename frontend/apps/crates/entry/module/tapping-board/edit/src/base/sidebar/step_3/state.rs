use crate::base::state::{Step, Base};
use std::rc::Rc;
use std::cell::RefCell;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt}
};
use dominator::clone;
use components::{
    image::search::state::{State as ImageSearchState, ImageSearchOptions},
    audio_input::{
        options::AudioInputOptions,
        state::State as AudioInputState,
        callbacks::Callbacks as AudioCallbacks,
    },
    stickers::state::Stickers,
};
use dominator_helpers::futures::AsyncLoader;

use shared::domain::jig::module::body::Audio;

pub struct Step3 {
    pub base: Rc<Base>,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let _self = Rc::new(Self {
            base,
        });

        _self
    }

    //The tab kind state is re-generated when selecting or deselecting a trace
    pub fn selected_tab_signal(&self) -> impl Signal<Item = Mutable<Option<TabKind>>> {
        self.trace_index_signal()
            .map(|index| index.is_some())
            .dedupe()
            .map(|has_index| {
                if has_index {
                    let kind = match crate::debug::settings().interaction_tab {
                        Some(kind) => kind,
                        None => TabKind::Text
                    };
                    Mutable::new(Some(kind))
                } else {
                    Mutable::new(None)
                }
            })
    }

    //The tab signal is re-generated when either the tab is clicked (changing the kind_state)
    //or a new trace is selected
    pub fn tab_signal(&self, selected_tab_signal: impl Signal<Item = Option<TabKind>>) -> impl Signal<Item = Option<Tab>> {

        let base = self.base.clone();

        map_ref! {
            let kind = selected_tab_signal,
            let index = self.trace_index_signal()
                => move {
                    match (*kind, *index) {
                        (Some(kind), Some(index)) => {
                            Some(Tab::new(base.clone(), kind, index))
                        },
                        _ => None

                    }
                }
        }
    }

    pub fn trace_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.base.traces.selected_index.signal_cloned()
    }

}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Audio
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Audio => "audio",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Text(usize, Mutable<Option<String>>),
    Audio(Rc<AudioInputState>)
}

impl Tab {
    pub fn new(base: Rc<Base>, kind:TabKind, index: usize) -> Self {
        match kind {
            TabKind::Text => {
                let text = base.traces_meta.lock_ref()[index].text.get_cloned();
                Self::Text(index, Mutable::new(text))
            },
            TabKind::Audio => {
                let mut opts = AudioInputOptions::default();

                opts.audio = base.traces_meta.lock_ref()[index].audio.get_cloned();

                let callbacks = AudioCallbacks::new(
                    Some(clone!(base, index => move |audio:Audio| {
                        base.set_trace_meta_audio(index, Some(audio));
                    })),
                    Some(clone!(base, index => move || {
                        base.set_trace_meta_audio(index, None);
                    })),
                );

                let state = AudioInputState::new(opts, callbacks);

                Self::Audio(Rc::new(state))
            }
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text(_, _) => TabKind::Text,
            Self::Audio(_) => TabKind::Audio,
        }
    }
}

