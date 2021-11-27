use crate::base::state::Base;
use utils::{prelude::*, math::BoundsF64};
use components::traces::utils::TraceShapeExt;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use futures_signals::signal::{Mutable, SignalExt};
use unicode_segmentation::UnicodeSegmentation;
use rand::prelude::*;
use gloo_timers::callback::Timeout;

use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::{
    TalkType as RawTalkType,
    TalkTypeItem as RawTalkTypeItem
};

pub struct TalkType {
    pub base: Rc<Base>,
    pub raw: RawTalkType,
    pub items: Vec<Rc<TalkTypeItem>>,
    pub rng: RefCell<ThreadRng>,
}

impl TalkType {
    pub fn new(base: Rc<Base>, raw: RawTalkType) -> Rc<Self> {
        let mut rng = thread_rng();
        let items = raw.items.iter().map(|raw_item| TalkTypeItem::new(base.clone(), raw_item.clone(), &mut rng)).collect();

        let _self = Rc::new(Self { 
            base, 
            raw,
            items,
            rng: RefCell::new(rng),
        });

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
    Correct 
}

pub struct HintLetters {
    pub letters: Vec<HintLetter>,
    pub indices: Vec<usize>
}

pub struct HintLetter {
    pub letter: String,
    pub revealed: bool,
}

impl TalkTypeItem {
    pub fn new(base: Rc<Base>, raw: RawTalkTypeItem, rng: &mut ThreadRng) -> Rc<Self> {
        let mut bounds = raw.hotspot.shape.calc_bounds(None).expect_ji("could not calc bounds");

        let hint_letters = match raw.texts.as_ref() {
            Some(text) => {
                let letters:Vec<HintLetter> = text[0]
                    .graphemes(true)
                    .into_iter()
                    .map(|letter| {
                        HintLetter {
                            letter: letter.to_string(),
                            revealed: false
                        }
                    })
                    .collect();

                let mut indices:Vec<usize> = (0..letters.len()).collect();
                indices.shuffle(rng);

                RefCell::new(HintLetters {
                    letters,
                    indices
                })
            },
            None => {
                RefCell::new(HintLetters {
                    letters: Vec::new(),
                    indices: Vec::new()
                })
            }
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