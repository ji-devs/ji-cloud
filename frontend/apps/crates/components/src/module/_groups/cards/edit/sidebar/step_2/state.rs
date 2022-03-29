use crate::{
    module::_groups::cards::edit::state::*,
    tabs::MenuTabKind,
    theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks},
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::jig::JigUpdateDraftDataRequest,
    error::EmptyError,
};
use std::rc::Rc;
use utils::prelude::api_with_auth_empty;
use wasm_bindgen_futures::spawn_local;

use super::custom_background::CustomBackground;

pub const STR_CHANGE_BACKGROUND: &str = "Change background";

pub struct Step2<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub theme_selector: Rc<ThemeSelector>,
    pub custom_background: Mutable<Option<Rc<CustomBackground<RawData, E>>>>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
}

impl<RawData: RawDataExt, E: ExtraExt> Step2<RawData, E> {
    pub fn new(
        base: Rc<CardsBase<RawData, E>>,
        tab_kind: Mutable<Option<MenuTabKind>>,
    ) -> Rc<Self> {
        let callbacks = ThemeSelectorCallbacks::new(clone!(base => move |theme_id| {
            base.set_theme(theme_id);

            spawn_local(clone!(base => async move {
                let jig_id_string = base.jig_id.0.to_string();

                let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id_string);

                let req = JigUpdateDraftDataRequest {
                    theme: Some(theme_id),
                    ..JigUpdateDraftDataRequest::default()
                };

                let _ = api_with_auth_empty::<EmptyError, _>(
                    &path,
                    endpoints::jig::UpdateDraftData::METHOD,
                    Some(req),
                )
                    .await;
            }))
        }));
        let theme_selector = Rc::new(ThemeSelector::new(base.theme_id.read_only(), callbacks));

        Rc::new(Self {
            base,
            theme_selector,
            custom_background: Mutable::new(None),
            tab_kind,
        })
    }
}
