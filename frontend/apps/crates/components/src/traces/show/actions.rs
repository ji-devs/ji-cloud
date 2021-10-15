use super::state::*;

impl TracesShow {
    pub fn select_index(&self, index: usize) {
        if let Some(on_select) = self.on_select.as_ref() {
            (on_select)(index);

            self.selected_index.set(Some(index));
        }
    }

    pub fn deselect(&self) {
        self.selected_index.set(None);
    }
}
