use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::poster::{Mode as RawMode, Content as RawContent, ModuleData as RawData}}};
use super::{
    state::{Base,Step},
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use crate::state::Mode;
use futures_signals::signal::{ReadOnlyMutable, Mutable};

pub fn init_from_mode(mode:Mode, history: Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let step = Mutable::new(Step::default());
    let base = Rc::new(Base::new(history, step.read_only()));
    
    StepsInit {
        step,
        base: base.clone(),
        main: Main::new(base.clone()),
        sidebar: Sidebar::new(base.clone()),
        header: Header::new(base.clone()),
        footer: Footer::new(base.clone()),
        overlay: Overlay::new(base.clone()),
    }
}

pub fn init_from_raw(raw:RawData, is_history: bool, history: Rc<HistoryStateImpl<RawData>>) -> Option<StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>> {
    raw.content.map(|content| {
        //TODO - create from raw
        let step = Mutable::new(Step::default());
        let base = Rc::new(Base::new(history, step.read_only()));
        
        let mut init = StepsInit {
            step,
            base: base.clone(),
            main: Main::new(base.clone()),
            sidebar: Sidebar::new(base.clone()),
            header: Header::new(base.clone()),
            footer: Footer::new(base.clone()),
            overlay: Overlay::new(base.clone()),
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
}
