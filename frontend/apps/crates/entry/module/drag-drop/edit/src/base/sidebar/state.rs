use crate::base::{
    main::{drag::MainDrag, select::MainSelect},
    state::Base,
};
use components::{module::_common::edit::prelude::*, tabs::MenuTabKind};
use std::rc::Rc;

use futures_signals::signal::{Mutable, Signal};

pub struct Sidebar {
    pub base: Rc<Base>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
    pub sticker_phase: Mutable<Option<StickerPhase>>,
    pub trace_phase: Mutable<Option<TracePhase>>,
}

impl Sidebar {
    pub fn new(base: Rc<Base>) -> Self {
        Self {
            base,
            tab_kind: Mutable::new(None),
            sticker_phase: Mutable::new(None),
            trace_phase: Mutable::new(None),
        }
    }
}

impl SidebarExt for Sidebar {
    type TabKindSignal = impl Signal<Item = Option<MenuTabKind>>;

    fn tab_kind(&self) -> Self::TabKindSignal {
        self.tab_kind.signal()
    }
}

#[derive(Clone)]
pub enum StickerPhase {
    Scene,
    Select(Rc<MainSelect>),
    Drag(Rc<MainDrag>),
    Static,
}

impl PartialEq for StickerPhase {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Scene, Self::Scene) => true,
            (Self::Select(_), Self::Select(_)) => true,
            (Self::Drag(_), Self::Drag(_)) => true,
            (Self::Static, Self::Static) => true,
            _ => false,
        }
    }
}

impl Eq for StickerPhase {}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TracePhase {
    Edit,
    Show,
}
