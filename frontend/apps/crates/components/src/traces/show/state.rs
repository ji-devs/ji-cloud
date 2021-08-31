use shared::domain::jig::module::body::_groups::design::Trace;
use futures_signals::signal::{Mutable, Signal, SignalExt};
use std::rc::Rc;

pub struct TracesShow {
    pub traces: Vec<Trace>,
    pub mode: Mutable<TracesShowMode>,
    pub on_select: Option<Box<dyn Fn(usize) + 'static>>,
    pub selected_index: Mutable<Option<usize>>,

}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum TracesShowMode {
    Cutout,
    Solid,
    Hidden,
}

impl TracesShow {
    pub fn new(traces: Vec<Trace>, mode: TracesShowMode, on_select: Option<impl Fn(usize) + 'static>) -> Rc<Self> {
        Rc::new(Self {
            traces,
            mode: Mutable::new(mode),
            on_select: on_select.map(|f| Box::new(f) as _),
            selected_index: Mutable::new(None),
        })
    }

    pub fn on_select_noop() -> Option<impl Fn(usize) + 'static> {
        None::<fn(usize)>
    }

    pub fn get_trace(&self, index: usize) -> &Trace {
        &self.traces[index]
    }
}
