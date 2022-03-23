use crate::base::state::Base;
use std::rc::Rc;

use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};

use super::super::state::Sidebar;
use shared::domain::jig::module::body::Audio;

pub struct Step3 {
    pub sidebar: Rc<Sidebar>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        Rc::new(Self { sidebar })
    }

    //The tab kind state is re-generated when selecting or deselecting a trace
    pub fn selected_tab_signal(&self) -> impl Signal<Item = Mutable<Option<MenuTabKind>>> {
        self.trace_index_signal()
            .map(|index| index.is_some())
            .dedupe()
            .map(|has_index| {
                if has_index {
                    let kind = match crate::debug::settings().interaction_tab {
                        Some(kind) => kind,
                        None => MenuTabKind::Audio,
                    };
                    Mutable::new(Some(kind))
                } else {
                    Mutable::new(None)
                }
            })
    }

    //The tab signal is re-generated when either the tab is clicked (changing the kind_state)
    //or a new trace is selected
    pub fn tab_signal(
        &self,
        selected_tab_signal: impl Signal<Item = Option<MenuTabKind>>,
    ) -> impl Signal<Item = Option<Tab>> {
        let base = self.sidebar.base.clone();

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
        self.sidebar.base.traces.selected_index.signal_cloned()
    }
}

#[derive(Clone)]
pub enum Tab {
    Label(usize, Mutable<Option<String>>),
    Audio(Rc<AudioInput>),
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind, index: usize) -> Self {
        match kind {
            MenuTabKind::Label => {
                let text = base.traces.get_text(index);
                Self::Label(index, Mutable::new(text))
            }
            MenuTabKind::Audio => {
                let opts = AudioInputOptions::new(Some(base.traces.audio_signal(index)));

                let callbacks = AudioInputCallbacks::new(
                    Some(clone!(base, index => move |audio:Audio| {
                        base.traces.set_audio(index, Some(audio));
                    })),
                    Some(clone!(base, index => move || {
                        base.traces.set_audio(index, None);
                    })),
                );

                let state = AudioInput::new(opts, callbacks);

                Self::Audio(state)
            }

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn _kind(&self) -> MenuTabKind {
        match self {
            Self::Label(_, _) => MenuTabKind::Label,
            Self::Audio(_) => MenuTabKind::Audio,
        }
    }
}
