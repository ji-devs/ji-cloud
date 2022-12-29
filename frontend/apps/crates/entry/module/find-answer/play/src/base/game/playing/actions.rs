use super::state::*;
use components::{
    audio::mixer::{AudioPath, AUDIO_MIXER},
    module::_common::play::prelude::{BaseExt, ModuleEnding, ModulePlayPhase},
};
use dominator::clone;
use std::rc::Rc;
use utils::prelude::*;

impl PlayState {
    pub fn select(state: Rc<Self>, index: usize) {
        // Mark the selected set
        state.selected_set.lock_mut().insert(index);

        state.clone().evaluate_question_ended();

        // Play the correct sound effect always. But we also need to make sure that it is finished playing before
        // moving on to the next activity.
        state.play_correct_sound(clone!(state => move || {
            for (trace_index, trace) in state.traces.iter().enumerate() {
                if trace_index == index {
                    trace.select(state.clone());
                } else {
                    trace.kill_playback();
                }
            }
        }));
    }

    pub fn incorrect_choice(state: Rc<Self>, _incorrect_index: Option<usize>) {
        state
            .incorrect_choice_count
            .set(state.incorrect_choice_count.get() + 1);
        AUDIO_MIXER.with(move |mixer| {
            let audio_path: AudioPath<'_> = mixer.get_random_negative().into();
            mixer.play_oneshot_on_ended(audio_path, move || {
                // TODO once the advanced modal has been added, negative feedback audio can be added here.
            });
        });
    }

    fn play_correct_sound<R, F: Fn() -> R + 'static>(self: &Rc<Self>, f: F) {
        AUDIO_MIXER.with(move |mixer| {
            let audio_path: AudioPath<'_> = mixer.get_random_positive().into();
            mixer.play_oneshot_on_ended(audio_path, move || {
                f();
            });
        });
    }

    pub fn evaluate_question_ended(self: Rc<Self>) {
        // In the next iteration of this, we'll be adding the ability to configure incorrect traces. For that
        // we'll need to probably update the selected_set to include selected trace kinds so that we can filter
        // for only traces which are correct.
        let selected_traces = self.selected_set.get_cloned().len();
        let total_traces = self.question.traces.len();

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
