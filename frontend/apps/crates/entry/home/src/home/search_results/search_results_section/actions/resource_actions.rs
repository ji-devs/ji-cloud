use std::rc::Rc;

use shared::{api::endpoints, domain::resource::ResourceSearchPath};
use utils::prelude::ApiEndpointExt;

use crate::home::search_results::search_results_section::SearchResultsSection;

impl SearchResultsSection {
    pub async fn load_resources(self: &Rc<Self>) {
        let mut req = self.home_state.search_selected.to_resource_search_request();

        req.page = Some(self.next_page.get());

        match endpoints::resource::Search::api_no_auth(ResourceSearchPath(), Some(req)).await {
            Err(_) => todo!(),
            Ok(res) => {
                let mut resources = self.list.lock_mut();
                res.resources.into_iter().for_each(|resource| {
                    resources.push_cloned(Rc::new(resource.into()));
                });

                self.total.set(res.total_resource_count);

                let mut last_page_loaded = self.next_page.lock_mut();
                *last_page_loaded += 1;
            }
        };
    }
}
