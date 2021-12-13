use std::rc::Rc;

use shared::{domain::jig::{JigSearchQuery, JigSearchResponse}, error::EmptyError, api::{endpoints, ApiEndpoint}};
use utils::prelude::api_no_auth;

use super::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_items(self: &Rc<Self>, mut req: JigSearchQuery) {
        req.jig_focus = Some(self.focus);

        req.page = Some(self.next_page.get());

        match api_no_auth::<JigSearchResponse, EmptyError, JigSearchQuery>(
            endpoints::jig::Search::PATH,
            endpoints::jig::Search::METHOD,
            Some(req),
        )
        .await
        {
            Err(_) => todo!(),
            Ok(res) => {
                self.fill_from_response(res);
            }
        };
    }
}
