use super::state::Text;

impl Text {
    pub fn set_value(&self, value: String) {
        self.value.set_neq(value);
    }

}

