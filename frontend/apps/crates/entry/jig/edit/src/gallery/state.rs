use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{jig::{JigFocus, JigResponse}, meta::AgeRange};
use strum_macros::{Display, EnumIter, EnumString};

pub const TEMPLATE_KINDS: &[&str] = &[
    "vocabulary",
    "parsha",
    "parsha",
    "vocabulary",
    "parsha",
    "parsha",
];

#[derive(Debug, Clone, PartialEq, EnumIter, Display, EnumString)]
pub enum VisibleJigs {
    All,
    Published,
    Draft,
}

pub struct JigGallery {
    pub loader: AsyncLoader,
    pub jigs: MutableVec<JigResponse>,
    pub visible_jigs: Rc<Mutable<VisibleJigs>>,
    pub age_ranges: Mutable<Vec<AgeRange>>,
}

impl JigGallery {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            loader: AsyncLoader::new(),
            jigs: MutableVec::new(),
            visible_jigs: Rc::new(Mutable::new(VisibleJigs::All)),
            age_ranges: Mutable::new(vec![]),
        })
    }
}
