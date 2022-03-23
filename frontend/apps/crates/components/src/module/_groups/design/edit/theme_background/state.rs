use dominator::clone;
use futures_signals::signal::Mutable;
use shared::api::{endpoints, ApiEndpoint};
use shared::domain::jig::JigUpdateDraftDataRequest;
use shared::domain::jig::module::body::StepExt;
use shared::error::EmptyError;
use utils::prelude::api_with_auth_empty;
use wasm_bindgen_futures::spawn_local;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::module::_groups::design::edit::design_ext::DesignExt;
use crate::tabs::MenuTabKind;
use crate::theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks};
use crate::module::_common::edit::entry::prelude::BaseExt;

use super::custom_background::CustomBackground;

pub const STR_DESIGN_FROM_SCRATCH: &str = "Design from scratch";

pub struct ThemeBackground<Step, Base> where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub base: Rc<Base>,
    pub theme_selector: Rc<ThemeSelector>,
    pub custom_background: Mutable<Option<Rc<CustomBackground<Step, Base>>>>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
    _step: PhantomData<Step>,
}

impl<Step, Base> ThemeBackground<Step, Base> where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static,
{
    pub fn new(base: Rc<Base>, tab_kind: Mutable<Option<MenuTabKind>>) -> Rc<Self> {

        let callbacks = ThemeSelectorCallbacks::new(clone!(base => move |theme_id| {
            base.set_theme(theme_id);

            spawn_local(clone!(base => async move {
                let jig_id = base.get_jig_id().0.to_string();

                let path = endpoints::jig::UpdateDraftData::PATH.replace("{id}", &jig_id);

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
        let theme_selector = Rc::new(ThemeSelector::new(
            base.get_theme().read_only(),
            callbacks,
        ));

        Rc::new(Self {
            base,
            theme_selector,
            custom_background: Mutable::new(None),
            tab_kind,
            _step: PhantomData,
        })
    }
}
