use super::state::*;
use shared::domain::jig::module::body::ThemeChoice;
use shared::{
    api::endpoints::{self, ApiEndpoint},
    domain::jig::JigUpdateRequest,
    error::EmptyError,
};

use utils::prelude::*;

impl ThemeSelector {
    pub fn set_theme(&self, theme: ThemeChoice) {
        (self.callbacks.on_change)(theme);
    }

    pub fn set_jig_theme_id(&self, theme_id: ThemeId) {
        self.jig_theme_id.set_neq(theme_id);

        let path = endpoints::jig::Update::PATH.replace("{id}", &self.jig_id.0.to_string());

        let req = JigUpdateRequest {
            theme: Some(theme_id),
            ..JigUpdateRequest::default()
        };

        self.jig_id_saver.load(async move {
            api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::Update::METHOD, Some(req))
                .await
                .unwrap_ji();
        });
    }
}
