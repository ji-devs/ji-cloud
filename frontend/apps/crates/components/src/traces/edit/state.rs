use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal},
    signal_vec::MutableVec,
};

use std::rc::Rc;

use shared::domain::jig::module::body::_groups::design::Trace as RawTrace;

use super::{select::trace::state::*, callbacks::*, draw::state::*};
use crate::traces::utils::TraceExt;
use utils::resize::get_resize_info;

pub struct TracesEdit {
    pub list: MutableVec<Rc<EditSelectTrace>>,
    pub selected_index: Mutable<Option<usize>>,
    pub phase: Mutable<Phase>,
    pub callbacks: Callbacks,
}

#[derive(Clone)]
pub enum Phase {
    Selectable,
    Draw(Rc<Draw>),
}

#[derive(Clone, Debug, Default)]
pub struct DebugOptions {
    pub start_in_phase_draw: bool,
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
        debug_opts: Option<DebugOptions>,
        callbacks: Callbacks,
    ) -> Rc<Self> {
        let debug_opts = debug_opts.unwrap_or_default();

        let _self = Rc::new(Self {
            list: MutableVec::new(),
            selected_index: Mutable::new(None),
            phase: Mutable::new(Phase::Selectable),
            callbacks,
        });

        if raw.len() > 0 {
            let resize_info = get_resize_info();
            _self.list.lock_mut().replace_cloned(
                raw.into_iter()
                    .map(|trace| Rc::new(EditSelectTrace::new(trace.clone(), &resize_info)))
                    .collect(),
            );
        }

        if debug_opts.start_in_phase_draw {
            Self::start_draw(_self.clone(), None, None);
        }

        _self
    }

    pub fn get_current(&self) -> Option<Rc<EditSelectTrace>> {
        self.selected_index.get_cloned().and_then(|i| self.get(i))
    }

    pub fn get(&self, index: usize) -> Option<Rc<EditSelectTrace>> {
        self.list.lock_ref().get(index).map(|x| x.clone())
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
