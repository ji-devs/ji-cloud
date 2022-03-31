pub struct SendResetLinkCallbacks {
    pub on_close: Box<dyn Fn()>,
}

impl SendResetLinkCallbacks {
    pub fn new(on_close: impl Fn() + 'static) -> Self {
        Self {
            on_close: Box::new(on_close),
        }
    }
}
