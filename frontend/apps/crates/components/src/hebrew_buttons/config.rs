use super::Kind;

pub struct HebrewButtonsConfig {
    pub kind: Kind,
    pub on_open_toggle: Box<dyn Fn(bool)>
}

impl Default for HebrewButtonsConfig {
    fn default() -> Self {
        Self {
            kind: Kind::Full,
            on_open_toggle: Box::new(|_b|{}),
        }
    }
}
