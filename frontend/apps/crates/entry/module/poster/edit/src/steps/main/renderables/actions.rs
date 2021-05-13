use super::state::*;

impl Renderables {
    pub fn delete_index(&self, index: usize) {
        self.list.lock_mut().remove(index);
        self.call_update();
        /*
        self.get_history().push_modify(|game_data| {
            game_data.pairs.remove(pair_index);
        });
        */
    }

    fn call_update(&self) {
        if let Some(on_updated) = self.on_updated.borrow().as_ref() {
            //TODO - get vec of raw renderables
            on_updated(vec![]);
        }
    }
}
