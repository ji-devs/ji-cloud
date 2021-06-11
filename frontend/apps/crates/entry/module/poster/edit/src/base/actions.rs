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
            poster::{Mode, Content as RawContent, ModuleData as RawData}
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
