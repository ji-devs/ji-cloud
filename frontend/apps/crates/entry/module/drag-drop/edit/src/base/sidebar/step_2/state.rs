use crate::base::state::*;
use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    tabs::MenuTabKind,
};
use dominator::clone;
use dominator_helpers::signals::{rc_signal_fn, RcSignalFn};
use futures_signals::signal::{Mutable, SignalExt};
use shared::domain::jig::module::body::Audio;
use std::rc::Rc;

use super::super::state::Sidebar;

pub struct Step2 {
    pub tab: Mutable<Tab>,
    pub sidebar: Rc<Sidebar>,
}

impl Step2 {
    pub fn new(sidebar: Rc<Sidebar>) -> Rc<Self> {
        let kind = match crate::debug::settings().step_2_tab {
            Some(kind) => kind,
            None => MenuTabKind::Select,
        };

        let tab = Mutable::new(Tab::new(sidebar.base.clone(), kind));

        Rc::new(Self { sidebar, tab })
    }
}

#[derive(Clone)]
pub enum Tab {
    Select,
    Audio(RcSignalFn<Option<Rc<AudioInput>>>),
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

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Select => MenuTabKind::Select,
            Self::Audio(_) => MenuTabKind::Audio,
        }
    }
}
