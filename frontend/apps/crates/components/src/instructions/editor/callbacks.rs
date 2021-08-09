use shared::domain::jig::module::body::Instructions;

pub struct Callbacks {
    pub save: Box<dyn Fn(Instructions, bool)>, //the flag indicates whether it should be pushed to history too
}

impl Callbacks {
    pub fn new(save: impl Fn(Instructions, bool) + 'static) -> Self {
        Self {
            save: Box::new(save),
        }
    }
}
