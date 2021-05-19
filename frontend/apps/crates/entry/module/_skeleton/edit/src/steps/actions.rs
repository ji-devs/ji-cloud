use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::poster::{Mode as RawMode, Content as RawContent, ModuleData as RawData}}};
use super::{
    state::{Sections,Step},
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use crate::state::Mode;

pub fn init_from_mode(mode:Mode, history: Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay> {
    let sections = Rc::new(Sections::new(history));
    
    StepsInit {
        step: None,
        sections,
        main: Main::new(),
        sidebar: Sidebar::new(),
        header: Header::new(),
        footer: Footer::new(),
        overlay: Overlay::new(),
    }
}

pub fn init_from_raw(raw:RawData, is_history: bool, history: Rc<HistoryStateImpl<RawData>>) -> Option<StepsInit<Step, Sections, Main, Sidebar, Header, Footer, Overlay>> {
    raw.content.map(|content| {
        //TODO - create from raw
        let sections = Rc::new(Sections::new(history));
        
        let mut init = StepsInit {
            step: None,
            sections,
            main: Main::new(),
            sidebar: Sidebar::new(),
            header: Header::new(),
            footer: Footer::new(),
            overlay: Overlay::new(),
        };

        if !is_history {
            init.step = crate::debug::settings().step;
        }

        init
    })
}

impl Sections {
    pub fn stub_action(&self) {
        self.history.push_modify(move |raw| {
            
        });
    }
}
