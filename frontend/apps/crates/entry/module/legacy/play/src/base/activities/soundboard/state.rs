use crate::base::{activities::_common::hotspot::*, state::Base};
use dominator::clone;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use shared::domain::jig::module::body::legacy::activity::{
    Soundboard as RawSoundboard, SoundboardItem as RawSoundboardItem,
};
use std::rc::Rc;
pub struct Soundboard {
    pub base: Rc<Base>,
    pub raw: RawSoundboard,
    pub items: Vec<Rc<SoundboardItem>>,
    pub phase: Mutable<Phase>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Intro,
    Hints,
    Playing,
}

impl Soundboard {
    pub fn new(base: Rc<Base>, raw: RawSoundboard) -> Rc<Self> {
        let items = raw
            .items
            .iter()
            .map(|raw_item| SoundboardItem::new(base.clone(), raw_item))
            .collect();

        let _self = Rc::new(Self {
            base,
            raw,
            items,
            phase: Mutable::new(Phase::Intro),
        });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}

pub struct SoundboardItem {
    pub base: Rc<Base>,
    pub audio_filename: Option<String>,
    pub text: Option<String>,
    pub jump_index: Option<usize>,
    pub hotspot: Rc<Hotspot>,
    pub revealed: Mutable<bool>,
}

impl SoundboardItem {
    pub fn new(base: Rc<Base>, raw: &RawSoundboardItem) -> Rc<Self> {
        let hotspot = Hotspot::new(raw.hotspot.clone());

        Rc::new(Self {
            audio_filename: raw.audio_filename.clone(),
            text: raw.text.clone(),
            jump_index: raw.jump_index,
            base,
            hotspot,
            revealed: Mutable::new(false),
        })
    }

    pub fn _text_signal(&self) -> impl Signal<Item = Option<String>> {
        let text = self.text.clone();
        self.revealed.signal().map(clone!(text => move |revealed| {
            if revealed {
                text.clone()
            } else {
                None
            }
        }))
    }
}
