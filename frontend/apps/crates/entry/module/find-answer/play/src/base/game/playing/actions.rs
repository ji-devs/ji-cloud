use super::state::*;
use components::module::_common::play::prelude::*;
use shared::domain::module::body::find_answer::Next;
use std::rc::Rc;

impl PlayState {
    pub fn select(state: Rc<Self>, index: usize) {
        // mark the selected set
        state.selected_set.lock_mut().insert(index);

        for (trace_index, trace) in state.traces.iter().enumerate() {
            if trace_index == index {
                trace.select(state.clone());
            } else {
                trace.kill_playback();
            }
        }
    }

    /// Evaluate whether the _question_ is complete. If it is, move on to the next question.
    pub fn evaluate_end(&self) {
        // In the next iteration of this, we'll be adding the ability to configure incorrect traces. For that
        // we'll need to probably update the selected_set to include selected trace kinds so that we can filter
        // for only traces which are correct.
        let selected_traces = self.selected_set.get_cloned().len();
        let total_traces = self.question.traces.len();

        // Return early if the student still has traces left to select. No point in continuing with other checks.
        if selected_traces < total_traces {
            return;
        }

        // We need to know how many questions have been completed so that if the settings have the
        // activity ending after _n_ questions, we can move on to the next activity.
        let n_questions_completed = match self.game.question.get_cloned() {
            Some((index, ..)) => index + 1,
            None => 0,
        };

        let total_questions = self.game.base.questions.len();
        let n_target = {
            match self.game.base.settings.next {
                Next::SelectAll => Some(total_questions),
                Next::SelectSome(n) => Some(n),
                Next::Continue => None,
            }
        };

        // This gets complex - We have three possible transitions here...
        // 1. The student hasn't completed enough questions, and we can move on to the next question;
        // 2. or, they have completed the required amount of questions, and we move to the next activity;
        // 3. or, they need to click continue to move on the next activity.
        //
        // For #3, we don't handle any state change.
        let completed_minimum = match n_target {
            // Have they completed the minimum required questions?
            Some(n_target) if n_questions_completed >= n_target => true,
            // They haven't, or the settings don't require them to.
            _ => false,
        };
        let next_question = self.game.next_question_index();

        match (completed_minimum, next_question) {
            // If they've completed the minimum, then we can end this activity
            (true, _) => {
                self.game
                    .base
                    .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
            }
            // If they haven't completed the minimum questions, or don't need to, _and_ we can move on to the next
            // question
            (false, Some(next_index)) => {
                // TODO Add a delay before transitioning to the next question
                self.game.move_next_question(next_index);
            }
            (_, None) => {
                // No more questions to ask, but the activity is configured so that the student can click continue.
                //
                // Do nothing.
            }
        }
    }
}
