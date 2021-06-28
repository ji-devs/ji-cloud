use shared::domain::jig::module::body::_groups::design::Trace as RawTrace;

pub struct Callbacks {
    pub on_add: Option<Box<dyn Fn(RawTrace)>>,
    pub on_delete: Option<Box<dyn Fn(usize)>>,
    pub on_change: Option<Box<dyn Fn(usize, RawTrace)>>,
}

impl Callbacks {
    pub fn new(
        on_add: Option<impl Fn(RawTrace) + 'static>,
        on_delete: Option<impl Fn(usize) + 'static>,
        on_change: Option<impl Fn(usize, RawTrace) + 'static>,
    ) -> Self {
        Self {
            on_add: on_add.map(|f| Box::new(f) as _),
            on_delete: on_delete.map(|f| Box::new(f) as _),
            on_change: on_change.map(|f| Box::new(f) as _),
        }
    }
}
