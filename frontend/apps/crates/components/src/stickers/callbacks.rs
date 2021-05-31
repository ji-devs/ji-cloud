use shared::domain::jig::module::body::Sticker as RawSticker;

pub struct Callbacks {
    pub on_change: Option<Box<dyn Fn(Vec<RawSticker>)>>,
}

impl Callbacks {
    pub fn new(
        on_change: Option<impl Fn(Vec<RawSticker>) + 'static>,
    ) -> Self {
        Self {
            on_change: on_change.map(|f| Box::new(f) as _),
        }
    }
}
