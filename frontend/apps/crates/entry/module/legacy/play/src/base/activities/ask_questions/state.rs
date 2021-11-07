use std::{cell::RefCell, rc::Rc, sync::atomic::{AtomicU8, Ordering}};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use gloo_timers::callback::Timeout;
use shared::domain::jig::module::body::legacy::activity::{AskQuestions as RawAskQuestions, QuestionItem as RawQuestionItem};
use dominator::clone;
use utils::unwrap::UnwrapJiExt;
use crate::base::{
    state::Base,
    activities::_common::hotspot::*
};
pub struct AskQuestions {
    pub base: Rc<Base>,
    pub raw: RawAskQuestions,
    //in stack-order (i.e. reverse of input)
    pub item_bank: RefCell<Vec<RawQuestionItem>>,
    pub item: Mutable<Rc<QuestionItem>>,
    pub phase: Mutable<Phase>,
    pub wrong_count: AtomicU8
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Play,
    Hint,
    WaitingNext 
}

impl AskQuestions {
    pub fn new(base: Rc<Base>, raw: RawAskQuestions) -> Rc<Self> {
        let mut item_bank:Vec<RawQuestionItem> = raw.items.iter().cloned().rev().collect();
        let item = item_bank.pop().unwrap_ji();
        let item = Mutable::new(QuestionItem::new(base.clone(), item));

        let _self = Rc::new(Self{
            base,
            raw,
            item,
            item_bank: RefCell::new(item_bank),
            phase: Mutable::new(Phase::Play),
            wrong_count: AtomicU8::new(0),
        });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }
}

pub struct QuestionItem {
    pub base: Rc<Base>,
    pub question_filename: Option<String>,
    pub answer_filename: Option<String>,
    pub wrong_filename: Option<String>,
    pub hotspot: Rc<Hotspot>,
    pub revealed: Mutable<bool>,
    pub re_ask_timer: RefCell<Option<Timeout>>
}

impl QuestionItem {
    pub fn new(base: Rc<Base>, raw: RawQuestionItem) -> Rc<Self> {
        let hotspot = Hotspot::new(raw.hotspot);

        Rc::new(Self{
            question_filename: raw.question_filename,
            answer_filename: raw.answer_filename,
            wrong_filename: raw.wrong_filename,
            base,
            hotspot,
            revealed: Mutable::new(false),
            re_ask_timer: RefCell::new(None)
        })
    }
}