use shared::domain::jig::module::body::Audio;

pub struct Callbacks {
    pub on_new_text: Option<Box<dyn Fn(&str)>>,
    pub on_change: Option<Box<dyn Fn(&str)>>,
    pub on_blur: Option<Box<dyn Fn()>>,
}

impl Callbacks {
    pub fn new(
        on_new_text: Option<impl Fn(&str) + 'static>, 
        on_change: Option<impl Fn(&str) + 'static>, 
        on_blur: Option<impl Fn() + 'static>, 
    ) -> Self {
        Self {
            on_new_text: on_new_text.map(|f| Box::new(f) as _),
            on_change: on_change.map(|f| Box::new(f) as _),
            on_blur: on_blur.map(|f| Box::new(f) as _),
        }
    }
}
