use shared::domain::module::body::Image;

pub struct ImageSearchCallbacks {
    pub on_select: Option<Box<dyn Fn(Option<Image>)>>,
}

impl ImageSearchCallbacks {
    pub fn new(on_select: Option<impl Fn(Option<Image>) + 'static>) -> Self {
        Self {
            on_select: on_select.map(|f| Box::new(f) as _),
        }
    }
}
