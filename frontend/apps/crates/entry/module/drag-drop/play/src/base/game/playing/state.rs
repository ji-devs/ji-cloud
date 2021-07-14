use std::{rc::Rc, cell::RefCell};
use crate::base::game::state::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Signal, SignalExt, Mutable}
};
use components::{
    traces::{
        utils::TraceExt,
        bubble::state::TraceBubble,
    }
};
use utils::{prelude::*, drag::Drag};
use shared::domain::jig::module::body::{Audio, Transform, _groups::design::{Sticker, Trace}, drag_drop::{Interactive, ItemKind, Next}};
use web_sys::AudioContext;
use std::collections::HashSet;

pub struct PlayState {
    pub game: Rc<Game>,
    pub items: Vec<PlayItem>
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let items = game.base.items
            .iter()
            .map(|item| {
                let item = item.clone();

                match item.kind {
                    ItemKind::Static => PlayItem::Static(item.sticker),
                    ItemKind::Interactive(data) => {
                        PlayItem::Interactive(InteractiveItem::new(item.sticker, data))
                    }
                }
            })
            .collect();

        Rc::new(Self {
            game,
            items,
        })
    }
}

pub enum PlayItem {
    Static(Sticker),
    Interactive(Rc<InteractiveItem>)
}

pub struct InteractiveItem {
    pub sticker: Sticker,
    pub audio: Option<Audio>,
    pub target_transform: Transform, 
    pub curr_transform: Mutable<Transform>,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub size: Mutable<Option<(f64, f64)>>,
}

impl InteractiveItem {
    pub fn new(sticker: Sticker, data: Interactive) -> Rc<Self> {
        let transform = sticker.transform().clone();
        Rc::new(Self {
            sticker,
            audio: data.audio,
            target_transform: data.target_transform.unwrap_or_else(clone!(transform => move || transform)),
            curr_transform: Mutable::new(transform),
            drag: Mutable::new(None),
            size: Mutable::new(None),
        })
    }
}

