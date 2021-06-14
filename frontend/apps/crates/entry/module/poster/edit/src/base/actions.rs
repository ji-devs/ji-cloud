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
            poster::{Mode, Step, Content as RawContent, ModuleData as RawData}
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
use futures_signals::signal::{ReadOnlyMutable, Mutable};
use utils::prelude::*;
use components::{
    text_editor::state::State as TextEditorState,
    audio_mixer::AudioMixer,
};

pub async fn init_from_raw(
    audio_mixer: AudioMixer,
    step_mutables: ReadOnlyStepMutables<Step>,
    jig_id: JigId,
    module_id: ModuleId,
    jig: Option<Jig>,
    raw:RawData, 
    init_source: InitSource, 
    history: Rc<HistoryStateImpl<RawData>>
) -> StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let force_step = {
        if init_source == InitSource::ForceRaw { 
            crate::debug::settings().step
        } else {
            None
        }
    };


    let base = Base::new(audio_mixer, jig_id, module_id, jig, raw, step_mutables.0, history).await;
    
    StepsInit {
        force_step,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base.clone())),
    }
}