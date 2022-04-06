use crate::base::game::state::*;
use components::audio::mixer::AudioHandle;
use dominator::clone;
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{self, SignalVecExt},
};
use shared::domain::jig::module::body::{
    Audio, Transform,
    _groups::design::Sticker,
    drag_drop::{Interactive, ItemKind},
};
use std::{cell::RefCell, rc::Rc};
use utils::drag::Drag;

use components::collision::stickers_traces::pixels::{StickerBoundsKind, StickerHitSource};
use components::instructions::player::InstructionsPlayer;
use std::borrow::Cow;
pub struct PlayState {
    pub game: Rc<Game>,
    pub items: Vec<PlayItem>,
    pub feedback_player: Mutable<Option<Rc<InstructionsPlayer>>>,
}

impl PlayState {
    pub fn new(game: Rc<Game>) -> Rc<Self> {
        let items = game
            .base
            .items
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
            feedback_player: Mutable::new(None),
        })
    }

    pub fn all_interactive_items_have_sizes(&self) -> impl Signal<Item = bool> {
        signal_vec::always(
            self.items
                .iter()
                .filter_map(|item| match item {
                    PlayItem::Interactive(item) => Some(item.clone()),
                    _ => None,
                })
                .collect::<Vec<Rc<InteractiveItem>>>(),
        )
        .filter_signal_cloned(|data| data.size.signal_cloned().map(|size| size.is_none()))
        .is_empty()
        .dedupe()
    }
}

pub enum PlayItem {
    Static(Sticker),
    Interactive(Rc<InteractiveItem>),
}

pub struct InteractiveItem {
    pub sticker: Sticker,
    pub completed: Mutable<bool>,
    pub audio: Option<Audio>,
    pub audio_effect_handle: RefCell<Option<AudioHandle>>,
    pub audio_user_handle: RefCell<Option<AudioHandle>>,
    pub target_transform: Transform,
    pub curr_transform: Mutable<Transform>,
    pub drag: Mutable<Option<Rc<Drag>>>,
    pub size: Mutable<Option<(f64, f64)>>,
    pub target_index: RefCell<Option<usize>>,
}

pub enum SourceTransformOverride {
    Current,
    Target,
}

impl InteractiveItem {
    pub fn new(sticker: Sticker, data: Interactive) -> Rc<Self> {
        let transform = sticker.transform().clone();
        Rc::new(Self {
            sticker,
            completed: Mutable::new(false),
            audio: data.audio,
            target_transform: data
                .target_transform
                .unwrap_or_else(clone!(transform => move || transform)),
            curr_transform: Mutable::new(transform),
            drag: Mutable::new(None),
            size: Mutable::new(None),
            target_index: RefCell::new(None),
            audio_effect_handle: RefCell::new(None),
            audio_user_handle: RefCell::new(None),
        })
    }

    pub fn get_hit_source(
        &self,
        transform_override: Option<SourceTransformOverride>,
    ) -> Option<StickerHitSource> {
        self.size.get_cloned().map(|size| {
            let transform_override = transform_override.map(|t| match t {
                SourceTransformOverride::Current => {
                    let transform = self.curr_transform.get_cloned();
                    Cow::Owned(transform)
                }
                SourceTransformOverride::Target => Cow::Borrowed(&self.target_transform),
            });

            StickerHitSource {
                sticker: Cow::Borrowed(&self.sticker),
                size,
                transform_override,
                bounds_kind: StickerBoundsKind::Auto,
            }
        })
    }
}
