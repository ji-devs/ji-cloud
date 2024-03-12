use super::state::*;
use components::{
    audio::mixer::{AudioPath, AUDIO_MIXER},
    module::_common::play::prelude::{BaseExt, ModuleEnding, ModulePlayPhase},
};
use dominator::clone;
use shared::domain::{jig::codes::JigPlaySessionModule, module::body::_groups::design::TraceKind};
use std::rc::Rc;
use utils::{prelude::*, toasts};

impl PlayState {
    pub fn select(state: Rc<Self>, index: usize) {
        let selected_trace = state.traces.get(index).unwrap_ji();

        // If an incorrect trace is selected that doesn't have audio associated, then don't highlight it.
        if selected_trace.kind != TraceKind::Wrong
            || selected_trace.audio.is_some()
            || state.question.incorrect_audio.is_some()
        {
            state.selected_set.lock_mut().insert(index);
        }

        state.clone().evaluate_question_ended();

        state.kill_all_playback();

        match selected_trace.kind {
            TraceKind::Correct => {
                state.incorrect_choice_count.set(0);
                state.correct_choice(clone!(state, selected_trace => move || {
                    let has_audio = if selected_trace.inner.audio.is_some() {
                        true
                    } else {
                        if let Some(audio) = &state.question.correct_audio {
                            state.selection_audio.set(Some(audio.clone()));
                            true
                        } else {
                            false
                        }
                    };

                    // To get around FnOnce requirements for TraceBubble
                    fn evaluate_end(state: &Rc<PlayState>) {
                        state.clone().evaluate_end();
                    }

                    if !has_audio {
                        // evaluate_end is called by the dom too, but only when audio playback ends.
                        // If there is no correct audio for the question, we need to make sure that the
                        // question ends correctly.
                        evaluate_end(&state);
                    }

                    selected_trace.select(state.clone(), Some(evaluate_end));
                }));
            }
            TraceKind::Wrong => {
                state.incorrect_choice(clone!(state, selected_trace => move || {
                    if selected_trace.inner.audio.is_none() {
                        if let Some(audio) = &state.question.incorrect_audio {
                            state.selection_audio.set(Some(audio.clone()));
                        }
                    }
                    selected_trace.select(state.clone(), None)
                }));
            }
            _ => {}
        }
    }

    pub fn kill_all_playback(self: &Rc<Self>) {
        // Make sure that all playback is stopped so that we can play any new audio for the
        // latest selection.
        self.selected_set
            .lock_mut()
            .iter()
            .for_each(|index| self.traces.get(*index).unwrap_ji().kill_playback());
    }

    pub fn incorrect_choice<F: Fn() + 'static>(self: &Rc<Self>, f: F) {
        if let Some((index, _)) = &*self.game.base.current_question.lock_ref() {
            self.game
                .base
                .play_report
                .lock_mut()
                .items
                .get_mut(*index)
                .unwrap_ji()
                .failed_tries += 1;
        }

        let state = self;
        state
            .incorrect_choice_count
            .set(state.incorrect_choice_count.get() + 1);

        AUDIO_MIXER.with(clone!(state => move |mixer| {
            let audio_path: AudioPath<'_> = mixer.get_random_negative().into();
            *state.selection_audio_handle.borrow_mut() = Some(mixer.play_on_ended(audio_path, false, clone!(state => move || {
                *state.selection_audio_handle.borrow_mut() = None;
                f()
            })));
        }));
    }

    pub fn remove_incorrect_highlights(self: Rc<Self>) {
        let state = self;
        state
            .clone()
            .selected_set
            .lock_mut()
            .retain(clone!(state => move |index| state.traces.get(*index).unwrap_ji().kind == TraceKind::Correct));
    }

    fn correct_choice<F: Fn() + 'static>(self: &Rc<Self>, f: F) {
        let state = self;

        if let Some((index, _)) = &*self.game.base.current_question.lock_ref() {
            let points = calculate_point_count(
                state
                    .game
                    .base
                    .play_report
                    .lock_mut()
                    .items
                    .get(*index)
                    .unwrap_ji()
                    .failed_tries as u32,
            );
            let _ = IframeAction::new(ModuleToJigPlayerMessage::AddPoints(points))
                .try_post_message_to_player();
        }

        AUDIO_MIXER.with(clone!(state => move |mixer| {
            let audio_path: AudioPath<'_> = mixer.get_random_positive().into();
            *state.selection_audio_handle.borrow_mut() = Some(mixer.play_on_ended(audio_path, false, clone!(state => move || {
                *state.selection_audio_handle.borrow_mut() = None;
                f()
            })));
        }));
    }

    pub fn evaluate_question_ended(self: Rc<Self>) {
        // We only want to evaluate _correct_ answers. Incorrect answers are included in the
        // selected_traces field, so filter them out. Same applies to the questions traces.
        let selected_traces = self
            .selected_set
            .get_cloned()
            .iter()
            .filter(|trace| {
                self.question.traces.get(**trace).unwrap_ji().kind == TraceKind::Correct
            })
            .count();
        let total_traces = self
            .question
            .traces
            .iter()
            .filter(|trace| trace.kind == TraceKind::Correct)
            .count();

        // Return early if the student still has traces left to select. No point in continuing with other checks.
        if selected_traces < total_traces {
            return;
        }

        // Stop the timer from ticking once a question is done
        IframeAction::new(ModuleToJigPlayerMessage::PauseTimer)
            .try_post_message_to_player()
            .unwrap_ji();

        // At this point, we know that the student has selected all the traces and that this question is done.
        self.ended.set(true);
    }

    /// Evaluate whether the _question_ is complete. If it is, move on to the next question.
    pub fn evaluate_end(self: Rc<Self>) {
        if self.ended.get() {
            let next_question = self.game.next_question_index();
            let state = self;

            match next_question {
                // We can move on to the next question if one exists
                Some(next_index) => {
                    state.game.base.move_to_question(next_index);
                    state.ended.set_neq(false);
                }
                // Otherwise, there are no more questions to ask, move on to the next activity, or play the feedback
                None => {
                    let info = state.game.base.play_report.lock_ref().clone();
                    let info = JigPlaySessionModule::FindAnswer(info);
                    let msg = IframeAction::new(ModuleToJigPlayerMessage::AddCodeSessionInfo(info));
                    if msg.try_post_message_to_player().is_err() {
                        toasts::error("Error saving progress");
                        log::info!("Error saving progress");
                    }

                    let feedback = &state.game.base.feedback;
                    if feedback.has_content() {
                        state.game.base.feedback_signal.set(Some(feedback.clone()));
                    } else {
                        state
                            .game
                            .base
                            .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
                    }
                }
            }
        }
    }
}

fn calculate_point_count(tried_count: u32) -> u32 {
    // start with 2 point, reduce one point for every try. min points: 0.
    let base = 2_u32;
    base.saturating_sub(tried_count)
}
