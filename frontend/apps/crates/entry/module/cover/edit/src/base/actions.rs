use super::{
    footer::state::Footer, header::state::Header, main::state::Main, overlay::state::Overlay,
    sidebar::state::Sidebar, state::*,
};
use components::module::_common::edit::prelude::*;
use shared::domain::jig::module::{body::{cover::{ModuleData as RawData, Step, Content}, _groups::design::BaseContent}, ModuleUpdateRequest, StableOrUniqueId, ModuleBody};
use utils::prelude::api_with_auth_empty;
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
        overlay: Rc::new(Overlay::new(base)),
    }
}

pub async fn set_contents_with_theme(jig_id: JigId, module_id: ModuleId, theme: ThemeId, mut module: RawData) -> RawData {
    log::info!("{:?}", theme);
    let content = Content {
        base: BaseContent {
            theme: theme.clone(),
            ..Default::default()
        },
        ..Default::default()
    };

    module.content = Some(content);

    let body = ModuleBody::Cover(module.clone());

    let req = ModuleUpdateRequest {
        id: StableOrUniqueId::Unique(module_id),
        body: Some(body),
        index: None,
        is_complete: None,
    };

    let path = endpoints::jig::module::Update::PATH
        .replace("{id}", &jig_id.0.to_string())
        .replace("{module_id}", &module_id.0.to_string());

    let _ = api_with_auth_empty::<EmptyError, _>(
        &path,
        endpoints::jig::module::Update::METHOD,
        Some(req)
    )
        .await;

    module
}
