use super::state::*;
use components::audio::mixer::{AudioPath, AUDIO_MIXER};
use gloo_timers::callback::Timeout;
use std::{rc::Rc, sync::atomic::Ordering};
use crate::base::actions::NavigationTarget;

use dominator::clone;

impl AskQuestions {
    pub fn on_start(self: Rc<Self>) {
        self.item.lock_ref().clone().start_asking();
    }

    pub fn on_bg_click(self: Rc<Self>) {
        if self.phase.get() != Phase::WaitingNext {
            let item = self.item.get_cloned();
            item.on_wrong_click(self);
        }
    }

    pub fn next_question(self: Rc<Self>) {
        self.wrong_count.store(0, Ordering::SeqCst);

        match self.item_bank.borrow_mut().pop() {
            None => {
                log::info!("all finished!");
                self.base.navigate(NavigationTarget::Next);
            }
            Some(item) => {
                let item = QuestionItem::new(self.base.clone(), item);
                item.clone().start_asking();
                self.item.set(item);

                self.phase.set_neq(Phase::Play);
            }
        }
    }
}

impl QuestionItem {
    pub fn on_correct_click(self: Rc<Self>, parent: Rc<AskQuestions>) {
        let state = self;

        parent.phase.set_neq(Phase::WaitingNext);

        let url = match state.answer_filename.as_ref() {
            Some(x) => state.base.activity_media_url(x),
            None => AudioPath::from(AUDIO_MIXER.with(|mixer| mixer.get_random_positive())).url(),
        };

        state.stop_asking();
        state.base.audio_manager.play_clip_on_ended(
            url,
            clone!(parent => move || {
                parent.clone().next_question();
            }),
        );
    }

    pub fn on_wrong_click(self: Rc<Self>, parent: Rc<AskQuestions>) {
        let state = self;

        let url = match state.wrong_filename.as_ref() {
            Some(x) => state.base.activity_media_url(x),
            None => AudioPath::from(AUDIO_MIXER.with(|mixer| mixer.get_random_negative())).url(),
        };

        state.stop_asking();
        state.base.audio_manager.play_clip_on_ended(
            url,
            clone!(state => move || {
                *state.re_ask_timer.borrow_mut() = Some(Timeout::new(crate::config::ASK_QUESTION_WAIT_RE_ASK, clone!(state => move || {
                    state.start_asking();
                })))
            })
        );

        let count = parent.wrong_count.fetch_add(1, Ordering::SeqCst);
        if count >= crate::config::WRONG_TRIES_UNTIL_HINT {
            parent.phase.set_neq(Phase::Hint);
            parent.wrong_count.store(0, Ordering::SeqCst);
        }
    }

    pub fn stop_asking(&self) {
        self.base.audio_manager.stop_clip();
        *self.re_ask_timer.borrow_mut() = None;
    }

    pub fn start_asking(self: Rc<Self>) {
        let state = self;

        state.base.audio_manager.stop_clip();
        if let Some(audio_filename) = &state.question_filename {
            state.base.audio_manager.play_clip_on_ended(
                state.base.activity_media_url(&audio_filename),
                clone!(state => move || {
                    *state.re_ask_timer.borrow_mut() = Some(Timeout::new(crate::config::ASK_QUESTION_WAIT_RE_ASK, move || {
                        //state.clone().start_asking();
                    }))
                })
            );
        }
    }
}
