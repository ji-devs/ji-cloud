use super::state::*;
use shared::{
    api::endpoints::{ApiEndpoint, self}, 
    domain::jig::{Jig, JigId, JigResponse, JigUpdateRequest}, 
    error::EmptyError
};
use shared::domain::jig::module::body::ThemeChoice;
use wasm_bindgen_futures::spawn_local;
use utils::prelude::*;

impl ThemeSelector {
    pub fn set_theme(&self, theme: ThemeChoice) {
        (self.callbacks.on_change) (theme);
    }

    pub fn set_jig_theme_id(&self, theme_id: ThemeId) {

        self.jig_theme_id.set_neq(theme_id);

        let path = endpoints::jig::Update::PATH
            .replace("{id}", &self.jig_id.0.to_string());

        let req = JigUpdateRequest {
            theme: Some(theme_id),
            ..JigUpdateRequest::default()
        };
        
        self.jig_id_saver.load(async move {
            api_with_auth_empty::<EmptyError, _>(&path, endpoints::jig::Update::METHOD, Some(req)).await.unwrap_ji();
        });
    }
}
