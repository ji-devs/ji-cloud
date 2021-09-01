use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Audio, _groups::design::{Trace as RawTrace, TraceKind}};

use super::{select::trace::state::*, draw::state::*};
use crate::traces::utils::TraceExt;
use utils::resize::get_resize_info;

pub struct TracesEdit {
    pub list: MutableVec<Rc<EditSelectTrace>>,
    pub selected_index: Mutable<Option<usize>>,
    pub phase: Mutable<TracesEditPhase>,
    pub callbacks: TracesEditCallbacks,
    pub draw_kind: RefCell<TraceKind>,
}

pub struct TracesEditCallbacks {
    pub on_add: Option<Box<dyn Fn(RawTrace)>>,
    pub on_delete: Option<Box<dyn Fn(usize)>>,
    pub on_change: Option<Box<dyn Fn(usize, RawTrace)>>,
}
impl TracesEditCallbacks {
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

#[derive(Clone)]
pub enum TracesEditPhase {
    Selectable,
    Draw(Rc<Draw>),
}

impl TracesEdit {
    pub fn to_raw(&self) -> Vec<RawTrace> {
        self.list
            .lock_ref()
            .iter()
            .map(|trace| trace.to_raw())
            .collect()
    }

    pub fn from_raw(
        raw: &[RawTrace],
        draw_kind: TraceKind,
        callbacks: TracesEditCallbacks,
    ) -> Rc<Self> {

        let _self = Rc::new(Self {
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            phase: Mutable::new(TracesEditPhase::Selectable),
            callbacks,
            draw_kind: RefCell::new(draw_kind),
        });

        if raw.len() > 0 {
            let resize_info = get_resize_info();
            _self.list.lock_mut().replace_cloned(
                raw.into_iter()
                    .map(|trace| Rc::new(EditSelectTrace::new(trace.clone(), &resize_info)))
                    .collect(),
            );
        }

        _self
    }

    pub fn get_current(&self) -> Option<Rc<EditSelectTrace>> {
        self.selected_index.get_cloned().and_then(|i| self.get(i))
    }

    pub fn get(&self, index: usize) -> Option<Rc<EditSelectTrace>> {
        self.list.lock_ref().get(index).map(|x| x.clone())
    }

    pub fn get_audio(&self, index: usize) -> Option<Audio> {
        self.get(index).and_then(|trace| trace.audio.clone())
    }
    pub fn get_text(&self, index: usize) -> Option<String> {
        self.get(index).and_then(|trace| trace.text.clone())
    }

    pub fn audio_signal(&self, index: usize) -> impl Signal<Item = Option<Audio>> {
        self.list.signal_vec_cloned()
            .to_signal_map(move |traces| {
                traces
                    .get(index)
                    .and_then(|trace| trace.audio.clone())
            })
    }

    pub fn text_signal(&self, index: usize) -> impl Signal<Item = Option<String>> {
        self.list.signal_vec_cloned()
            .to_signal_map(move |traces| {
                traces
                    .get(index)
                    .and_then(|trace| trace.text.clone())
            })
    }

    pub fn selected_signal(
        &self,
        index: ReadOnlyMutable<Option<usize>>,
    ) -> impl Signal<Item = bool> {
        map_ref! {
            let index = index.signal(),
            let selected = self.selected_index.signal_cloned()
                => {
                    match (*index, *selected) {
                        (Some(index), Some(selected)) => {
                            index == selected
                        },
                        _ => false
                    }
                }
        }
    }
}
