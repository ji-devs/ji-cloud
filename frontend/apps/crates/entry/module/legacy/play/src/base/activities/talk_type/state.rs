use crate::base::state::Base;
use components::traces::utils::TraceShapeExt;
use futures_signals::signal::Mutable;
use rand::prelude::*;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use unicode_segmentation::UnicodeSegmentation;
use utils::{math::BoundsF64, prelude::*};

use dominator::clone;
use shared::domain::module::body::legacy::activity::{
    TalkType as RawTalkType, TalkTypeItem as RawTalkTypeItem,
};

pub struct TalkType {
    pub base: Rc<Base>,
    pub raw: RawTalkType,
    pub items: Vec<Rc<TalkTypeItem>>,
}

impl TalkType {
    pub fn new(base: Rc<Base>, raw: RawTalkType) -> Rc<Self> {
        let mut rng = thread_rng();
        let items = raw
            .items
            .iter()
            .map(|raw_item| TalkTypeItem::new(base.clone(), raw_item.clone(), &mut rng))
            .collect();

        let _self = Rc::new(Self { base, raw, items });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}

pub struct TalkTypeItem {
    pub base: Rc<Base>,
    pub raw: RawTalkTypeItem,
    pub bounds: BoundsF64,
    pub value: Mutable<String>,
    pub hint_letters: RefCell<HintLetters>,
    pub phase: Mutable<TalkTypeItemPhase>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TalkTypeItemPhase {
    Input,
    Wrong,
    Correct,
}

pub struct HintLetters {
    pub letters: Vec<HintLetter>,
    pub indices: Vec<usize>,
    pub largest_text: String,
}

pub struct HintLetter {
    pub letter: String,
    pub revealed: bool,
}

impl TalkTypeItem {
    pub fn new(base: Rc<Base>, raw: RawTalkTypeItem, rng: &mut ThreadRng) -> Rc<Self> {
        let bounds = raw
            .hotspot
            .shape
            .calc_bounds(None)
            .expect_ji("could not calc bounds");

        let hint_letters = match raw.texts.as_ref().and_then(|text| {
            let text: Vec<&String> = text.iter().filter(|text| !text.is_empty()).collect();

            if !text.is_empty() {
                Some(text)
            } else {
                None
            }
        }) {
            Some(text) => {
                let letters: Vec<HintLetter> = text[0]
                    .graphemes(true)
                    .into_iter()
                    .map(|letter| HintLetter {
                        letter: letter.to_string(),
                        revealed: false,
                    })
                    .collect();

                let mut indices: Vec<usize> = (0..letters.len()).collect();
                indices.shuffle(rng);

                let largest_text =
                    text.iter().fold(
                        "",
                        |acc, curr| {
                            if curr.len() > acc.len() {
                                curr
                            } else {
                                acc
                            }
                        },
                    );
                RefCell::new(HintLetters {
                    letters,
                    indices,
                    largest_text: largest_text.to_string(),
                })
            }
            None => RefCell::new(HintLetters {
                letters: Vec::new(),
                indices: Vec::new(),
                largest_text: "".to_string(),
            }),
        };
        Rc::new(Self {
            base,
            raw,
            bounds,
            value: Mutable::new("".to_string()),
            hint_letters,
            phase: Mutable::new(TalkTypeItemPhase::Input),
        })
    }
}

impl fmt::Display for HintLetters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for s in self.letters.iter() {
            if s.revealed {
                write!(f, "{}", s.letter)?;
            } else {
                write!(f, "_")?;
            }
        }
        Ok(())
        //write!(f, "({}, {})", self.x, self.y)
    }
}
