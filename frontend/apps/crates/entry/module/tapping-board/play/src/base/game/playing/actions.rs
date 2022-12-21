use super::state::*;
use components::module::_common::play::prelude::*;
use shared::domain::module::body::tapping_board::Next;
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

impl PlayState {
    pub fn select(state: Rc<Self>, index: usize) {
        // Kill playback for all already selected traces
        state
            .selected_set
            .lock_ref()
            .iter()
            .for_each(|index| state.traces.get(*index).unwrap_ji().kill_playback());

        // mark the selected set
        state.selected_set.lock_mut().insert(index);

        {
            let mut current_set = state.current_set.lock_mut();
            current_set.clear();
            current_set.insert(index);
        }

        state.traces.get(index).unwrap_ji().select(state.clone());
    }

    pub fn evaluate_end(&self) {
        let n_selected = self.selected_set.lock_ref().len();

        let n_target = {
            match self.game.base.settings.next {
                Next::SelectAll => Some(self.traces.len()),
                Next::Continue => None,
            }
        };

        if let Some(n_target) = n_target {
            if n_selected >= n_target {
                self.game
                    .base
                    .set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
            }
        }
    }
}
