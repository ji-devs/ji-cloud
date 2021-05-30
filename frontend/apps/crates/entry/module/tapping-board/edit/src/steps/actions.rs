use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    module::{
        ModuleId, 
        body::{
            Trace as RawTrace,
            Audio,
            tapping_board::{Mode as RawMode, TappingTrace, Content as RawContent, ModuleData as RawData}
        }
    }
};
use super::{
    state::*,
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use dominator::clone;
use crate::state::Mode;
use futures_signals::signal::{ReadOnlyMutable, Mutable};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
};

pub fn init_from_mode(mode:Mode, history: Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let step = Mutable::new(Step::default());
    let base = Base::new(false, history, step.read_only(), None);
    
    StepsInit {
        step,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base.clone())),
    }
}

pub fn init_from_raw(
    raw:RawData, 
    is_history: bool, 
    current: Option<Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>, 
    history: Rc<HistoryStateImpl<RawData>>
) -> Option<StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>> {
    raw.content.map(|content| {
        //TODO - create from raw
        let step = Mutable::new(Step::default());
        let base = Base::new(is_history, history, step.read_only(), Some(&content));
        
        let mut init = StepsInit {
            step,
            base: base.clone(),
            main: Rc::new(Main::new(base.clone())),
            sidebar: Rc::new(Sidebar::new(base.clone())),
            header: Rc::new(Header::new(base.clone())),
            footer: Rc::new(Footer::new(base.clone())),
            overlay: Rc::new(Overlay::new(base.clone())),
        };

        if !is_history {
            if let Some(step) = crate::debug::settings().step {
                init.step.set_neq(step);
            }
        }

        init
    })
}

impl Base {
    pub fn stub_action(&self) {
        self.history.push_modify(move |raw| {
            
        });
    }

    pub fn change_theme_id(&self, theme_id: ThemeId) {
        self.theme_id.set_neq(theme_id);

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.theme_id = theme_id;
            }
        });
    }

    /*
     * The traces themselves are managed by the component
     * Callbacks here are fired from there and need only to manage
     * meta and history
     */
    pub fn on_trace_added(&self, raw_trace: RawTrace) {
        self.traces_meta.lock_mut().push_cloned(TraceMeta::new(None, None));

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces.push(TappingTrace {
                    trace: raw_trace,
                    audio: None,
                    text: None,
                })
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

    pub fn on_trace_changed(&self, index: usize, raw_trace: RawTrace) {
        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces[index].trace = raw_trace;
            }
        });
    }

    pub fn set_trace_meta_audio(&self, index: usize, audio: Option<Audio>) {
        self.traces_meta.lock_ref().as_slice()[index].audio.set(audio.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces[index].audio = audio;
            }
        });
    }
    pub fn set_trace_meta_text(&self, index: usize, text: Option<String>) {
        self.traces_meta.lock_ref().as_slice()[index].text.set(text.clone());

        self.history.push_modify(move |raw| {
            if let Some(content) = &mut raw.content {
                content.traces[index].text = text;
            }
        });
    }
}
