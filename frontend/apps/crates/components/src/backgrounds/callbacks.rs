use shared::domain::jig::module::body::{Background, Backgrounds as RawBackgrounds};

pub struct Callbacks {
    pub on_change: Option<Box<dyn Fn(RawBackgrounds)>>,
}

impl Callbacks {
    pub fn new(
        on_change: Option<impl Fn(RawBackgrounds) + 'static>,
    ) -> Self {
        Self {
            on_change: on_change.map(|f| Box::new(f) as _),
        }
    }
}
