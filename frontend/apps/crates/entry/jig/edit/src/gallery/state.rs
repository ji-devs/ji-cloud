use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{
    asset::{Asset, AssetId},
    jig::JigFocus,
    meta::AgeRange,
};
use strum_macros::{Display, EnumIter, EnumString};

#[allow(dead_code)] // TODO: delete once used
pub const TEMPLATE_KINDS: &[&str] = &[
    "vocabulary",
    "parsha",
    "parsha",
    "vocabulary",
    "parsha",
    "parsha",
];

#[derive(Debug, Clone, PartialEq, EnumIter, Display, EnumString)]
pub enum VisibleAssets {
    All,
    Published,
    Draft,
}

pub struct Gallery {
    pub focus: JigFocus,
    pub loader: AsyncLoader,
    pub assets: MutableVec<Asset>,
    /// Total assets that can be fetched
    pub total_asset_count: Mutable<Option<u64>>,
    /// The next page to call when request a page of JIGs
    pub next_page: Mutable<u32>,
    pub visible_assets: Rc<Mutable<VisibleAssets>>,
    pub age_ranges: Mutable<Vec<AgeRange>>,
    pub confirm_delete: Mutable<Option<AssetId>>,
}

impl Gallery {
    pub fn new(focus: JigFocus) -> Rc<Self> {
        Rc::new(Self {
            focus,
            loader: AsyncLoader::new(),
            assets: MutableVec::new(),
            total_asset_count: Mutable::new(None),
            next_page: Mutable::new(0),
            visible_assets: Rc::new(Mutable::new(VisibleAssets::All)),
            age_ranges: Mutable::new(vec![]),
            confirm_delete: Mutable::new(None),
        })
    }
}
