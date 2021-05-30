use crate::steps::state::{Step, Base};
use std::rc::Rc;
use std::cell::RefCell;
use futures_signals::{
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
    pub tab: Mutable<Option<Tab>>,
    pub tab_trace_index: RefCell<Option<usize>>,
    pub selected_reactor: AsyncLoader,
}


impl Step3 {
    pub fn new(base: Rc<Base>) -> Rc<Self> {

        let _self = Rc::new(Self {
            base,
            tab: Mutable::new(None),
            tab_trace_index: RefCell::new(None),
            selected_reactor: AsyncLoader::new()
        });

        //Need to re-create the tab body whenever
        //selected trace has changed
        //activate_tab() will validate and ensure no cycles
        //(it's also used by manually changing tab)
        _self.selected_reactor.load(
            _self.trace_index_signal()
                .for_each(clone!(_self => move |index| {
                    match index {
                        Some(index) => {
                            let curr_kind = _self.tab.lock_ref().as_ref().map(|tab| tab.kind());
                            let kind = match curr_kind {
                                None => {
                                    match crate::debug::settings().interaction_tab {
                                        Some(kind) => kind,
                                        None => TabKind::Text
                                    }
                                },
                                Some(kind) => kind
                            };

                            _self.activate_tab(kind);
                        },
                        None => {
                            _self.tab.set(None);
                            *_self.tab_trace_index.borrow_mut() = None;
                        }
                    }

                    async {}
                }))
        );

        _self
    }

    pub fn trace_index_signal(&self) -> impl Signal<Item = Option<usize>> {
        self.base.traces.selected_index.signal_cloned()
    }

    pub fn has_tab_signal(&self) -> impl Signal<Item = bool> {
        self.tab.signal_ref(|tab| tab.is_some())
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

