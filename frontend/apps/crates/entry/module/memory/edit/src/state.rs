use components::module::edit::*;
use super::steps::{
    actions::{init_from_mode, init_from_raw},
    state::{Base,Step},
    footer::state::Footer,
    header::state::Header,
    main::state::Main,
    overlay::state::Overlay,
    sidebar::state::Sidebar
};
use std::rc::Rc;
use shared::domain::jig::{JigId, module::{ModuleId, body::memory::{Mode as RawMode, ModuleData as RawData}}};

pub type AppState = GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>;


pub fn create_state(jig_id: JigId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(jig_id, module_id);

    let mut opts = StateOpts::new( jig_id, module_id);
    opts.force_raw = crate::debug::settings().data.clone(); 
    opts.is_main_scrollable = false;
    opts.skip_save_for_debug = crate::debug::settings().skip_save;
    opts.skip_load_jig = crate::debug::settings().skip_load_jig;

    AppState::new(
        opts,
        init_from_mode, //create steps when mode selected
        init_from_raw, //create steps when raw loaded or reset via history
    )
}


//need to have local Mode due to orphan rule
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Duplicate,
    WordsAndImages,
    BeginsWith,
    Lettering,
    Riddles,
    Opposites,
    Synonymns,
    Translate,
}

impl From<RawMode> for Mode {
    fn from(raw:RawMode) -> Self {
        match raw {

            RawMode::Duplicate => Self::Duplicate,
            RawMode::WordsAndImages => Self::WordsAndImages,
            RawMode::BeginsWith => Self::BeginsWith,
            RawMode::Lettering => Self::Lettering,
            RawMode::Riddles => Self::Riddles,
            RawMode::Opposites => Self::Opposites,
            RawMode::Synonymns => Self::Synonymns,
            RawMode::Translate => Self::Translate,
        }
    }
}
impl From<Mode> for RawMode {
    fn from(mode:Mode) -> Self {
        match mode {
            Mode::Duplicate => Self::Duplicate,
            Mode::WordsAndImages => Self::WordsAndImages,
            Mode::BeginsWith => Self::BeginsWith,
            Mode::Lettering => Self::Lettering,
            Mode::Riddles => Self::Riddles,
            Mode::Opposites => Self::Opposites,
            Mode::Synonymns => Self::Synonymns,
            Mode::Translate => Self::Translate,
        }
    }
}

impl ModeExt for Mode {
    fn get_list() -> Vec<Self> {
        vec![
            Self::Duplicate,
            Self::WordsAndImages,
            Self::BeginsWith,
            Self::Lettering,
            Self::Riddles,
            Self::Opposites,
            Self::Synonymns,
            Self::Translate,
        ]
    }

    fn title() -> &'static str {
        crate::strings::mode::STR_TITLE
    }

    fn module() -> &'static str {
        "memory"
    }

    fn as_str_id(&self) -> &'static str {
        match self {

            Self::Duplicate => "duplicate",
            Self::WordsAndImages => "words-images",
            Self::BeginsWith => "begins-with",
            Self::Lettering => "lettering",
            Self::Riddles => "riddles",
            Self::Opposites => "opposites",
            Self::Synonymns => "synonymns",
            Self::Translate => "translate",

        }
    }

    fn as_str_label(&self) -> &'static str {
        match self {
            Self::Duplicate => crate::strings::mode::STR_DUPLICATE, 
            Self::WordsAndImages => crate::strings::mode::STR_WORDS_IMAGES, 
            Self::BeginsWith => crate::strings::mode::STR_BEGINS_WITH, 
            Self::Lettering => crate::strings::mode::STR_LETTERING, 
            Self::Riddles => crate::strings::mode::STR_RIDDLES, 
            Self::Opposites => crate::strings::mode::STR_OPPOSITES, 
            Self::Synonymns => crate::strings::mode::STR_SYNONYMNS, 
            Self::Translate => crate::strings::mode::STR_TRANSLATE, 
        }
    }
}
