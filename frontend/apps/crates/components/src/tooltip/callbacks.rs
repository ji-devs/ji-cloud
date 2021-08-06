pub struct TooltipErrorCallbacks {
    pub on_close: Option<Box<dyn Fn()>>,
}

impl TooltipErrorCallbacks {
    pub fn new(on_close: Option<impl Fn() + 'static>) -> Self {
        Self {
            on_close: on_close.map(|f| Box::new(f) as _),
        }
    }
}

pub struct TooltipConfirmCallbacks {
    pub on_confirm: Option<Box<dyn Fn()>>,
    pub on_cancel: Option<Box<dyn Fn()>>,
}

impl TooltipConfirmCallbacks {
    pub fn new(
        on_confirm: Option<impl Fn() + 'static>,
        on_cancel: Option<impl Fn() + 'static>,
    ) -> Self {
        Self {
            on_confirm: on_confirm.map(|f| Box::new(f) as _),
            on_cancel: on_cancel.map(|f| Box::new(f) as _),
        }
    }
}
