use components::module::edit::*;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Audio,
            Instructions,
            memory::{Mode as RawMode, Content as RawContent, ModuleData as RawData}
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

pub async fn init_from_mode(jig_id: JigId, module_id: ModuleId, jig: Option<Jig>, mode:Mode, history: Rc<HistoryStateImpl<RawData>>) -> StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {

    let step = Mutable::new(Step::default());
    let base = Base::new(jig_id, module_id, jig, false, history, step.read_only(), ModeOrRaw::Mode(mode)).await;
    
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

pub async fn init_from_raw(
    jig_id: JigId,
    module_id: ModuleId,
    jig: Option<Jig>,
    raw:RawData, 
    is_history: bool, 
    current: Option<Rc<Steps<Step, Base, Main, Sidebar, Header, Footer, Overlay>>>, 
    history: Rc<HistoryStateImpl<RawData>>
) -> Option<StepsInit<Step, Base, Main, Sidebar, Header, Footer, Overlay>> {
    match raw.content {
        None => None,
        Some(content) => { 
            let step = Mutable::new(Step::default());
            let base = Base::new(jig_id, module_id, jig, is_history, history, step.read_only(), ModeOrRaw::Raw(content)).await;
            
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

            Some(init)
        }
    }
}

