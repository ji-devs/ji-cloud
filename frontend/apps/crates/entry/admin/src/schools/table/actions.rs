use super::SchoolTable;
use crate::schools::table::TableState;

use dominator::clone;
use shared::api::endpoints;
use shared::domain::admin::{
    AdminSchoolNamesPath, AdminVerifySchoolNamePath, ImportSchoolNamesPath,
    SearchSchoolNamesParams, VerifySchoolNameRequest,
};
use shared::domain::billing::SchoolNameId;
use shared::domain::{Page, PageLimit};
use std::rc::Rc;
use utils::prelude::ApiEndpointExt;
use web_sys::File;

impl SchoolTable {
    pub fn load_data(self: &Rc<Self>) {
        let state = Rc::clone(self);
        state.parent.loader.load(clone!(state => async move {
            state.load_schools().await;
        }));
    }

    pub async fn load_schools(self: &Rc<Self>) {
        let search_query = self.parent.search_filters.q.get_cloned();
        let req = SearchSchoolNamesParams {
            q: if search_query.is_empty() {
                None
            } else {
                Some(search_query)
            },
            verified: self.parent.search_filters.verified.get_cloned().as_value(),
            page: self.parent.search_filters.active_page.get_cloned(),
            page_limit: PageLimit::default(),
        };

        match endpoints::admin::SearchSchoolNames::api_with_auth(AdminSchoolNamesPath(), Some(req))
            .await
        {
            Err(_) => todo!(),
            Ok(res) => {
                self.total_pages.set(Some(res.pages));
                self.schools.lock_mut().replace_cloned(
                    res.school_names
                        .into_iter()
                        .map(|school| Rc::new(school))
                        .collect(),
                );
            }
        }
    }

    pub fn set_verified(self: &Rc<Self>, school_name_id: SchoolNameId, verified: bool) {
        let state = Rc::clone(self);
        state.parent.loader.load(clone!(state => async move {
            match endpoints::admin::VerifySchoolName::api_with_auth(AdminVerifySchoolNamePath(), Some(VerifySchoolNameRequest {
                school_name_id,
                verified,
            })).await {
                Err(error) => {
                    log::error!("Error: {error:?}");
                },
                Ok(_) => state.load_schools().await,
            }
        }));
    }

    pub fn upload_school_import_csv(self: &Rc<Self>, file: File) {
        let state = Rc::clone(self);
        state.uploading.set(true);
        state.parent.loader.load(clone!(state => async move {
            let file_text = wasm_bindgen_futures::JsFuture::from(file.text()).await.unwrap().as_string();
            match endpoints::admin::ImportSchoolNames::api_with_auth(ImportSchoolNamesPath(), file_text).await {
                Err(_) => todo!(),
                Ok(existing) => {
                    state.table_state.set(TableState::UploadResults(existing));
                }
            }
            state.uploading.set(false);
        }));
    }

    pub fn go_to_page(self: &Rc<Self>, page: Page) {
        let state = self;
        state.parent.loader.load(clone!(state => async move {
            state.parent.search_filters.set_active_page(page);
        }));
    }
}
