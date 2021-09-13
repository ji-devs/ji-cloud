use super::state::*;
use shared::domain::jig::module::body::tapping_board::Next;
use components::module::_common::play::prelude::*;
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

    pub fn evaluate_end(&self) {
        let n_selected = self.selected_set.lock_ref().len();

        let n_target = {
            match self.game.base.settings.next {
                Next::SelectAll => Some(self.traces.len()),
                Next::SelectSome(n) => Some(n),
                Next::Continue => None
            }
        };

        if let Some(n_target) = n_target {
            if n_selected >= n_target {
                self.game.base.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
            }
        }
    }
}