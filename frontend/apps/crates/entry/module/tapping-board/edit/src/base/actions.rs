use components::module::edit::prelude::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Trace as RawTrace,
            Audio,
            Instructions,
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
    audio_mixer::AudioMixer,
};

pub async fn init_from_raw(
    audio_mixer: AudioMixer,
    jig_id: JigId,
    module_id: ModuleId,
    jig: Option<Jig>,
    raw:RawData, 
    init_source: InitSource, 
    current: Option<Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>, 
    history: Rc<HistoryStateImpl<RawData>>
) -> StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let step = Mutable::new({
        let mut step = Step::default();
        if init_source == InitSource::ForceRaw { 
            if let Some(debug_step) = crate::debug::settings().step {
                step = debug_step;
            } 
        }

        step
    });

    let base = Base::new(audio_mixer, jig_id, module_id, jig, raw, step.read_only(), history).await;
    
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


impl Base {
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
