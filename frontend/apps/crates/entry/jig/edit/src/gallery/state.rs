use std::rc::Rc;

use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use shared::domain::{jig::{JigFocus, JigResponse, JigId}, meta::AgeRange};
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
pub enum VisibleJigs {
    All,
    Published,
    Draft,
}

pub struct JigGallery {
    pub focus: JigFocus,
    pub loader: AsyncLoader,
    pub jigs: MutableVec<JigResponse>,
    pub visible_jigs: Rc<Mutable<VisibleJigs>>,
    pub age_ranges: Mutable<Vec<AgeRange>>,
    pub confirm_delete: Mutable<Option<JigId>>,
}

impl JigGallery {
    pub fn new(focus: JigFocus) -> Rc<Self> {
        Rc::new(Self {
            focus,
            loader: AsyncLoader::new(),
            jigs: MutableVec::new(),
            visible_jigs: Rc::new(Mutable::new(VisibleJigs::All)),
            age_ranges: Mutable::new(vec![]),
            confirm_delete: Mutable::new(None),
        })
    }
}
