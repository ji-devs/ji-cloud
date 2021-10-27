use std::{cell::RefCell, rc::Rc};
use awsm_web::audio::AudioHandle;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::legacy::activity::{Soundboard as RawSoundboard, SoundboardItem as RawSoundboardItem};
use dominator::clone;
use crate::base::{
    state::Base,
    activities::_common::hotspot::*
};
pub struct Soundboard {
    pub base: Rc<Base>,
    pub raw: RawSoundboard,
    pub audio: RefCell<Option<AudioHandle>>,
    pub items: Vec<Rc<SoundboardItem>>
}

impl Soundboard {
    pub fn new(base: Rc<Base>, raw: RawSoundboard) -> Rc<Self> {
        let items = raw.items
            .iter()
            .map(|raw_item| {
                SoundboardItem::new(base.clone(), raw_item)
            })
            .collect();

        let _self = Rc::new(Self{
            base,
            raw,
            audio: RefCell::new(None),
            items
        });

        // TODO- soundboard listens on the foreground...
        // but still need to detect sticker triggers...
        // or maybe no foreground, just global window click?
        // _self.base.set_bg_listener(clone!(_self => move || {
        //     _self.clone().on_bg_click();
        // }));

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}

pub struct SoundboardItem {
    pub base: Rc<Base>,
    pub hotspot: Rc<Hotspot>,
    pub revealed: Mutable<bool>
}

impl SoundboardItem {
    pub fn new(base: Rc<Base>, raw: &RawSoundboardItem) -> Rc<Self> {
        let hotspot = Hotspot::new(raw.shape.clone());

        Rc::new(Self{
            base,
            hotspot,
            revealed: Mutable::new(false)
        })
    }
}