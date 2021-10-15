use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::module::_common::edit::prelude::*;
use shared::domain::jig::module::body::{
    _groups::design::Trace,
    tapping_board::{Mode, ModuleData as RawData, Step},
};
use std::rc::Rc;

pub async fn init_from_raw(
    init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
) -> BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {
    let force_step = {
        if init_args.source == InitSource::ForceRaw {
            crate::debug::settings().step
        } else {
            None
        }
    };

    let base = Base::new(init_args).await;

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base.clone())),
    }
}

impl Base {
    /*
     * The traces themselves are managed by the component
     * Callbacks here are fired from there and need only to manage
     * meta and history
     */
    pub fn on_trace_added(&self, trace: Trace) {
        self.traces_meta.lock_mut().push_cloned(TraceMeta::new());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces.push(trace);
            }
        });
    }

    pub fn on_trace_deleted(&self, index: usize) {
        self.traces_meta.lock_mut().remove(index);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces.remove(index);
            }
        });
    }

    pub fn on_trace_changed(&self, index: usize, raw_trace: Trace) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces[index] = raw_trace;
            }
        });
    }
}
