use std::rc::Rc;

use shared::domain::{jig::Jig, meta::AgeRange};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use strum_macros::{EnumIter, Display, EnumString};

pub const TEMPLATE_KINDS: &[&str] = &["vocabulary", "parsha", "parsha", "vocabulary", "parsha", "parsha"];

#[derive(Debug, Clone, PartialEq, EnumIter, Display, EnumString)]
pub enum VisibleJigs {
    All,
    Published,
    Draft,
}

pub struct State {
    pub loader: AsyncLoader,
    pub jigs: MutableVec<Jig>,
    pub visible_jigs: Rc<Mutable<VisibleJigs>>,
    pub age_ranges: Mutable<Vec<AgeRange>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            loader: AsyncLoader::new(),
            jigs: MutableVec::new(),
            visible_jigs: Rc::new(Mutable::new(VisibleJigs::All)),
            age_ranges: Mutable::new(vec![]),
        }
    }
}
