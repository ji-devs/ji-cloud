use shared::domain::jig::module::body::Image;

pub struct Callbacks {
    pub on_select: Option<Box<dyn Fn(Image)>>,
}

impl Callbacks {
    pub fn new(on_select: Option<impl Fn(Image) + 'static>) -> Self {
        Self {
            on_select: on_select.map(|f| Box::new(f) as _),
        }
    }
}
