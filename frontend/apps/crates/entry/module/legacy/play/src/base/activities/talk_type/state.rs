use crate::base::state::Base;
use utils::{prelude::*, math::BoundsF64};
use components::traces::utils::TraceShapeExt;
use std::rc::Rc;
use futures_signals::signal::{Mutable, SignalExt};

use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::{
    TalkType as RawTalkType,
    TalkTypeItem as RawTalkTypeItem
};

pub struct TalkType {
    pub base: Rc<Base>,
    pub raw: RawTalkType,
    pub items: Vec<Rc<TalkTypeItem>>
}

impl TalkType {
    pub fn new(base: Rc<Base>, raw: RawTalkType) -> Rc<Self> {
        let items = raw.items.iter().map(|raw_item| TalkTypeItem::new(raw_item.clone())).collect();

        let _self = Rc::new(Self { 
            base, 
            raw,
            items
        });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}

pub struct TalkTypeItem {
    pub raw: RawTalkTypeItem,
    pub bounds: BoundsF64,
    pub value: Mutable<String>
}

impl TalkTypeItem {
    pub fn new(raw: RawTalkTypeItem) -> Rc<Self> {
        let mut bounds = raw.hotspot.shape.calc_bounds(None).expect_ji("could not calc bounds");

        Rc::new(Self {
            raw,
            bounds,
            value: Mutable::new("".to_string())
        })
    }
}