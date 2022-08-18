use std::rc::Rc;

use shared::{
    api::{endpoints, ApiEndpoint},
    domain::jig::{JigSearchQuery, JigSearchResponse},
    error::EmptyError,
};
use utils::prelude::api_no_auth;

use crate::home::search_results::search_results_section::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_resources(self: &Rc<Self>) {
        let mut req = self.home_state.search_selected.to_resource_search_request();

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
                let mut resources = self.list.lock_mut();
                res.jigs.into_iter().for_each(|resource| {
                    resources.push_cloned(Rc::new(resource.into()));
                });

                self.total.set(res.total_jig_count);

                let mut last_page_loaded = self.next_page.lock_mut();
                *last_page_loaded += 1;
            }
        };
    }
}
