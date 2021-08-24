pub struct PreviewPopupCallbacks {
    pub close: Box<dyn Fn()>,
}

impl PreviewPopupCallbacks {
    pub fn new(
        close: impl Fn() + 'static,
    ) -> Self {
        Self {
            close: Box::new(close),
        }
    }
}
