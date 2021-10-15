use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::module::_common::edit::prelude::*;
use shared::domain::jig::module::body::cover::{ModuleData as RawData, Step};
use std::rc::Rc;

pub async fn init_from_raw(
    init_args: BaseInitFromRawArgs<RawData, (), Step>,
) -> BaseInit<Step, Base, Main, Sidebar, Header, Footer, Overlay> {
    let force_step = {
        if init_args.source == InitSource::ForceRaw {
            crate::debug::settings().step
        } else {
            None
        }
    };

    let base = Base::new(init_args).await;

    BaseInit {
        force_step,
        force_theme: None,
        base: base.clone(),
        main: Rc::new(Main::new(base.clone())),
        sidebar: Rc::new(Sidebar::new(base.clone())),
        header: Rc::new(Header::new(base.clone())),
        footer: Rc::new(Footer::new(base.clone())),
        overlay: Rc::new(Overlay::new(base.clone())),
    }
}
