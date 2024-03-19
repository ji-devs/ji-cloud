pub struct QrDialogCallbacks {
    pub on_close: Box<dyn Fn()>,
}

impl QrDialogCallbacks {
    pub fn new(on_close: impl Fn() + 'static) -> Self {
        Self {
            on_close: Box::new(on_close),
        }
    }
}
