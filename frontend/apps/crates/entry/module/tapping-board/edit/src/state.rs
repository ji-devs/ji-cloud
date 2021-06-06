use components::module::edit::prelude::*;
use super::base::{
    actions::init_from_raw,
    state::{Base,Step},
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::tapping_board::{Mode as RawMode, ModuleData as RawData}}};

pub type AppState = GenericState<Mode, Step, RawData, RawMode, Base, Main, Sidebar, Header, Footer, Overlay>;


pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 
    opts.is_main_scrollable = false;
    opts.skip_save_for_debug = crate::debug::settings().skip_save;
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(
        opts,
        init_from_raw, 
    )
}





#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Printables,
    TalkingPictures,
    Comics,
    Timeline,
    FamilyTree,
    Poster, 
}

impl From<RawMode> for Mode {
    fn from(raw:RawMode) -> Self {
        match raw {
            RawMode::Printables => Self::Printables,
            RawMode::TalkingPictures => Self::TalkingPictures,
            RawMode::Comics => Self::Comics,
            RawMode::Timeline => Self::Timeline,
            RawMode::FamilyTree => Self::FamilyTree,
            RawMode::Poster => Self::Poster,
        }
    }
}
impl From<Mode> for RawMode {
    fn from(mode:Mode) -> Self {
        match mode {
            Mode::Printables => Self::Printables,
            Mode::TalkingPictures => Self::TalkingPictures,
            Mode::Comics => Self::Comics,
            Mode::Timeline => Self::Timeline,
            Mode::FamilyTree => Self::FamilyTree,
            Mode::Poster => Self::Poster,
        }
    }
}

impl ModeExt<RawMode> for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Printables,
            Self::TalkingPictures,
            Self::Comics,
            Self::Timeline,
            Self::FamilyTree,
            Self::Poster,
        ]
    }

    fn title() -> &'static str {
        crate::strings::mode::STR_TITLE
    }

    fn module() -> &'static str {
        "tapping-board"
    }

    fn as_str_id(&self) -> &'static str {
        match self {
            Self::Printables => "printables",
            Self::TalkingPictures => "talking-pictures",
            Self::Comics => "comics",
            Self::Timeline => "timeline",
            Self::FamilyTree => "family-tree",
            Self::Poster => "poster",
        }
    }

    fn as_str_label(&self) -> &'static str {
        match self {
            Self::Printables => crate::strings::mode::STR_PRINTABLES_LABEL, 
            Self::TalkingPictures => crate::strings::mode::STR_TALKING_PICTURES_LABEL,
            Self::Comics => crate::strings::mode::STR_COMICS_LABEL,
            Self::Timeline => crate::strings::mode::STR_TIMELINE_LABEL,
            Self::FamilyTree => crate::strings::mode::STR_FAMILY_TREE_LABEL,
            Self::Poster => crate::strings::mode::STR_POSTER_LABEL
        }
    }


    fn to_raw(&self) -> RawMode {
        (*self).into()
    }
}
