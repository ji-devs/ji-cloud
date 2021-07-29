use shared::domain::jig::module::body::_groups::design::Sticker as RawSticker;
use super::state::*;

pub struct Callbacks <T: AsSticker> {
    pub on_change: Option<Box<dyn Fn(&[T])>>,
}

impl <T: AsSticker> Callbacks <T> {
    pub fn new(
        on_change: Option<impl Fn(&[T]) + 'static>,
    ) -> Self {
        Self {
            on_change: on_change.map(|f| Box::new(f) as _),
        }
    }
}
