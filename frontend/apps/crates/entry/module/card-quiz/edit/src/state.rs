use super::settings::{
    self, main::state::MainSettings, sidebar::state::SidebarSettings, state::Settings,
};
use components::module::{
    _common::edit::prelude::*,
    _groups::cards::edit::{
        footer::state::Footer as CardsFooter, header::state::Header as CardsHeader,
        main::state::Main as CardsMain, overlay::state::Overlay as CardsOverlay,
        sidebar::state::Sidebar as CardsSidebar, state::*,
    },
};
use dominator::Dom;
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            _groups::cards::{Mode, Step},
            card_quiz::{Content, ModuleData as RawData},
        },
        ModuleId,
    },
};
use std::rc::Rc;
use utils::prelude::*;

type GetSidebarSettings = fn(Rc<Base>) -> SidebarSettings;
type RenderSidebarSettings = fn(Rc<SidebarSettings>) -> Dom;
type GetMainSettings = fn(Rc<Base>) -> MainSettings;
type RenderMainSettings = fn(Rc<MainSettings>) -> Dom;

pub type Base = CardsBase<RawData, Extra>;
pub type Footer = CardsFooter<RawData, Extra>;
pub type Header = CardsHeader<RawData, Extra>;
pub type Overlay = CardsOverlay<RawData, Extra>;
pub type Main = CardsMain<RawData, Extra, GetMainSettings, RenderMainSettings, MainSettings>;
pub type Sidebar =
    CardsSidebar<RawData, Extra, GetSidebarSettings, RenderSidebarSettings, SidebarSettings>;

pub type AppState = GenericState<Mode, Step, RawData, Base, Main, Sidebar, Header, Footer, Overlay>;

pub struct Extra {
    pub settings: Rc<Settings>,
}
impl Extra {
    pub fn new(content: Content) -> Self {
        Self {
            settings: Rc::new(Settings::new(content)),
        }
    }
}

impl ExtraExt for Extra {}

pub fn create_state(asset_id: AssetId, module_id: ModuleId) -> Rc<AppState> {
    crate::debug::init(asset_id, module_id);
    let debug_settings = crate::debug::settings();

    let mut opts = StateOpts::new(asset_id, module_id);
    opts.force_raw = debug_settings.data.clone();
    opts.is_main_scrollable = true;
    opts.skip_save_for_debug = debug_settings.skip_save;
    opts.skip_load_jig = debug_settings.skip_load_jig;

    AppState::new(opts, init_from_raw)
}

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

    let debug_settings = crate::debug::settings();

    let extra = Extra::new(init_args.raw.content.as_ref().unwrap_ji().clone());

    let base = Base::new(init_args, extra, debug_settings.base.clone()).await;

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(
            base.clone(),
            MainSettings::new,
            settings::main::dom::render,
        )),
        sidebar: Rc::new(Sidebar::new(
            base.clone(),
            SidebarSettings::new,
            settings::sidebar::dom::render,
            true,
        )),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base)),
    }
}
