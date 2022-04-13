use super::super::state::Sidebar;
use crate::base::state::{Base, ItemKind};
use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    module::_common::prelude::Audio,
    tabs::MenuTabKind,
};
use dominator::clone;
use dominator_helpers::signals::{rc_signal_fn, RcSignalFn};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

pub struct Step3 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step3 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let tab = Mutable::new(Tab::new(sidebar.base.clone(), MenuTabKind::Select));

        Rc::new(Self { sidebar, tab })
    }

    pub fn next_kind(&self) -> Option<MenuTabKind> {
        match self.tab.get_cloned().kind() {
            MenuTabKind::Select => Some(MenuTabKind::Audio),
            MenuTabKind::Audio => Some(MenuTabKind::Trace),
            MenuTabKind::Trace => Some(MenuTabKind::Place),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Select,
    Audio(RcSignalFn<Option<Rc<AudioInput>>>),
    Trace,
    Place,
}

impl Tab {
    pub fn new(base: Rc<Base>, kind: MenuTabKind) -> Self {
        match kind {
            MenuTabKind::Select => Self::Select,
            MenuTabKind::Audio => {
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
                                            Some(clone!(base => move |audio: Audio| {
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
            MenuTabKind::Trace => Self::Trace,
            MenuTabKind::Place => Self::Place,
            _ => {
                log::info!("kind {:?}", kind);
                unimplemented!("unsupported tab kind!")
            }
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Select => MenuTabKind::Select,
            Self::Audio(_) => MenuTabKind::Audio,
            Self::Trace => MenuTabKind::Trace,
            Self::Place => MenuTabKind::Place,
        }
    }
}
