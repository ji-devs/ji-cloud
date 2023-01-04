use dominator_helpers::futures::AsyncLoader;
use shared::domain::user::{public_user::PublicUser, UserId};

#[derive(Clone)]
pub struct EditableUser {
    pub id: UserId,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub loader: AsyncLoader,
}

impl From<PublicUser> for EditableUser {
    fn from(user: PublicUser) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_name: user.given_name,
            last_name: user.family_name,
            loader: AsyncLoader::new(),
        }
    }
}

// impl EditableUser {
//     pub fn to_jig_update_request(&self) -> JigUpdateDraftDataRequest {
//         // don't include additional_resources here since they're handled in separately
//         JigUpdateDraftDataRequest {
//             display_name: Some(self.display_name.get_cloned()),
//             description: Some(self.description.get_cloned()),
//             other_keywords: Some(self.other_keywords.get_cloned()),
//             age_ranges: Some(self.age_ranges.get_cloned().into_iter().collect()),
//             language: Some(self.language.get_cloned()),
//             categories: Some(self.categories.get_cloned().into_iter().collect()),
//             affiliations: Some(self.affiliations.get_cloned().into_iter().collect()),
//             privacy_level: Some(self.privacy_level.get()),
//             ..Default::default()
//         }
//     }

//     pub fn to_update_admin_data_request(&self) -> JigUpdateAdminDataRequest {
//         JigUpdateAdminDataRequest {
//             rating: self.rating.get_cloned(),
//             blocked: Some(self.blocked.get()),
//             ..Default::default()
//         }
//     }

//     pub async fn save_draft(self: &Rc<Self>) {
//         let req = self.to_jig_update_request();
//         let res = endpoints::jig::UpdateDraftData::api_with_auth_empty(
//             JigUpdateDraftDataPath(self.id),
//             Some(req),
//         )
//         .await;
//         match res {
//             Ok(res) => res,
//             Err(_) => todo!(),
//         }
//     }

//     pub async fn save_admin_data(self: &Rc<Self>) {
//         let req = self.to_update_admin_data_request();
//         let res = endpoints::jig::JigAdminDataUpdate::api_with_auth_empty(
//             JigAdminDataUpdatePath(self.id),
//             Some(req),
//         )
//         .await;
//         match res {
//             Ok(res) => res,
//             Err(_) => todo!(),
//         }
//     }

//     pub async fn publish(self: &Rc<Self>) {
//         let res = endpoints::jig::Publish::api_with_auth_empty(JigPublishPath(self.id), None).await;
//         match res {
//             Ok(res) => res,
//             Err(_) => todo!(),
//         }
//     }

//     pub fn save_and_publish(self: &Rc<Self>) {
//         let state = self;
//         state.loader.load(clone!(state => async move {
//             join!(
//                 state.save_draft(),
//                 state.save_admin_data(),
//             );
//             state.publish().await;
//         }))
//     }
// }
