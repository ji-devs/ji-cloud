use shared::domain::module::body::ModuleAssist;

pub struct Callbacks {
    pub save: Box<dyn Fn(ModuleAssist, bool)>, //the flag indicates whether it should be pushed to history too
}

impl Callbacks {
    pub fn new(save: impl Fn(ModuleAssist, bool) + 'static) -> Self {
        Self {
            save: Box::new(save),
        }
    }
}
