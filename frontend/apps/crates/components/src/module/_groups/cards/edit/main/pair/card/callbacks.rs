pub struct CardCallbacks {
    pub on_click: Option<Box<dyn Fn()>>,
}

impl CardCallbacks {
    pub fn new(on_click: Option<impl Fn() + 'static>) -> Self {
        Self {
            on_click: on_click.map(|f| Box::new(f) as _),
        }
    }
}
