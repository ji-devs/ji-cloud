use std::rc::Rc;

use shared::domain::jig::module::body::{Audio, Transform, _groups::design::Trace as RawTrace};

use super::{draw::state::*, select::trace::state::*, state::*};
use crate::traces::utils::TraceExt;
use dominator::clone;
use utils::{prelude::*, resize::get_resize_info};

impl TracesEdit {
    pub fn delete_index(&self, index: usize) {
        self.selected_index.set(None);
        self.list.lock_mut().remove(index);
        if let Some(on_delete) = self.callbacks.on_delete.as_ref() {
            (on_delete)(index)
        }
    }

    pub fn duplicate(&self, index: usize) {
        let raw_trace = {
            let mut raw_trace = self.list.lock_ref().get(index).unwrap_ji().to_raw();
            raw_trace.transform.nudge_for_duplicate();

            raw_trace
        };

        self.add(raw_trace);
    }

    pub fn add(&self, raw_trace: RawTrace) {
        let resize_info = get_resize_info();
        let trace = Rc::new(EditSelectTrace::new(raw_trace.clone(), &resize_info));

        {
            let mut list = self.list.lock_mut();
            list.push_cloned(trace);
            self.select_index(list.len() - 1);
        }

        if let Some(on_add) = self.callbacks.on_add.as_ref() {
            (on_add)(raw_trace);
        }
    }

    pub fn set_audio(&self, index: usize, audio: Option<Audio>) {
        let raw_trace = {
            let mut raw_trace = self.list.lock_ref().get(index).unwrap_ji().to_raw();
            raw_trace.audio = audio;
            raw_trace
        };

        self.change(index, raw_trace);
    }

    pub fn set_text(&self, index: usize, text: Option<String>) {
        let raw_trace = {
            let mut raw_trace = self.list.lock_ref().get(index).unwrap_ji().to_raw();
            raw_trace.text = text;
            raw_trace
        };

        self.change(index, raw_trace);
    }

    pub fn change_transform(&self, index: usize, transform: Transform) {
        let raw_trace = {
            let mut raw_trace = self.list.lock_ref().get(index).unwrap_ji().to_raw();
            raw_trace.transform = transform;
            raw_trace
        };

        self.change(index, raw_trace);
    }

    pub fn change(&self, index: usize, raw_trace: RawTrace) {
        let resize_info = get_resize_info();
        let trace = Rc::new(EditSelectTrace::new(raw_trace.clone(), &resize_info));

        {
            let mut list = self.list.lock_mut();
            list.set_cloned(index, trace);
            self.select_index(index);
        }

        if let Some(on_change) = self.callbacks.on_change.as_ref() {
            (on_change)(index, raw_trace);
        }
    }

    pub fn select_index(&self, index: usize) {
        self.selected_index.set_neq(Some(index));
    }

    pub fn deselect(&self) {
        self.selected_index.set(None);
    }

    pub fn start_draw(
        _self: Rc<Self>,
        replace_index: Option<usize>,
        init_point: Option<(i32, i32)>,
    ) {
        _self.selected_index.set_neq(None);

        let init_trace_index = replace_index.and_then(|index| {
            _self
                .list
                .lock_ref()
                .get(index)
                .map(|trace| (index, trace.to_raw()))
        });

        let draw = Draw::new(
            init_trace_index,
            *_self.draw_kind.borrow(),
            clone!(_self => move |raw_trace| {
                //On finished
                if let Some(raw_trace) = raw_trace {
                    match replace_index {
                        None => _self.add(raw_trace),
                        Some(index) => _self.change(index, raw_trace)
                    }
                }
                _self.phase.set(TracesEditPhase::Selectable);
            }),
        );

        if let Some((x, y)) = init_point {
            draw.start_draw(x, y);
        }
        _self.phase.set(TracesEditPhase::Draw(Rc::new(draw)));
    }
}
