use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::asset::{Asset, AssetType};

pub struct LikedSection {
    pub asset_type: AssetType,
    pub loader: AsyncLoader,
    pub list: Rc<MutableVec<Rc<Asset>>>,
    pub total: Mutable<u64>,
    pub next_page: Mutable<u32>,
}

impl LikedSection {
    pub fn new(asset_type: AssetType) -> Rc<Self> {
        Rc::new(Self {
            asset_type,
            loader: AsyncLoader::new(),
            list: Rc::new(MutableVec::new()),
            total: Mutable::new(0),
            next_page: Mutable::new(0),
        })
    }

    pub fn all_loaded_signal(self: &Rc<Self>) -> impl Signal<Item = bool> {
        map_ref! {
            let list_len = self.list.signal_vec_cloned().len(),
            let total = self.total.signal() => move {
                *list_len == *total as usize
            }
        }
    }
}
