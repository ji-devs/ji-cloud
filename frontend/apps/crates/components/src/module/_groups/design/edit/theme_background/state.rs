use crate::module::_common::edit::entry::prelude::BaseExt;
use crate::module::_groups::design::edit::design_ext::DesignExt;
use crate::tabs::MenuTabKind;
use crate::theme_selector::state::{ThemeSelector, ThemeSelectorCallbacks};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::api::endpoints;
use shared::domain::asset::AssetId;
use shared::domain::jig::{JigUpdateDraftDataPath, JigUpdateDraftDataRequest};
use shared::domain::module::body::{ModeExt, StepExt};
use std::marker::PhantomData;
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;
use wasm_bindgen_futures::spawn_local;

use super::custom_background::CustomBackground;

pub const STR_DESIGN_FROM_SCRATCH: &str = "Design from scratch";

pub struct ThemeBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub base: Rc<Base>,
    pub theme_selector: Rc<ThemeSelector>,
    pub custom_background: Mutable<Option<Rc<CustomBackground<Step, Mode, Base>>>>,
    pub tab_kind: Mutable<Option<MenuTabKind>>,
    _step: PhantomData<Step>,
}

impl<Step, Mode, Base> ThemeBackground<Step, Mode, Base>
where
    Step: StepExt + 'static,
    Mode: ModeExt + 'static,
    Base: BaseExt<Step> + DesignExt<Mode> + 'static,
{
    pub fn new(base: Rc<Base>, tab_kind: Mutable<Option<MenuTabKind>>) -> Rc<Self> {
        let callbacks = ThemeSelectorCallbacks::new(clone!(base => move |theme_id| {
            base.set_theme(theme_id);

            // if asset is Jig update theme
            if let AssetId::JigId(jig_id) = base.get_asset_id() {
                spawn_local(async move {
                    let req = JigUpdateDraftDataRequest {
                        theme: Some(theme_id),
                        ..JigUpdateDraftDataRequest::default()
                    };

                    let _ = endpoints::jig::UpdateDraftData::api_with_auth_empty(JigUpdateDraftDataPath(jig_id.clone()), Some(req))
                        .await;
                })
            }
        }));
        let theme_selector = Rc::new(ThemeSelector::new(base.get_theme().read_only(), callbacks));

        Rc::new(Self {
            base,
            theme_selector,
            custom_background: Mutable::new(None),
            tab_kind,
            _step: PhantomData,
        })
    }
}
