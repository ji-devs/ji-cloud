use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use shared::domain::user::{UserBadge, UserId, UserResponse};

#[derive(Clone)]
pub struct EditableUser {
    pub id: UserId,
    pub username: Mutable<String>,
    pub first_name: Mutable<String>,
    pub last_name: Mutable<String>,
    pub email: Mutable<String>,
    pub signup_date: Mutable<String>,
    pub city: Mutable<String>,
    pub state: Mutable<String>,
    pub country: Mutable<String>,
    pub language: Mutable<String>,
    pub organization: Mutable<String>,
    pub badge: Mutable<Option<UserBadge>>,
    pub subscription: String,
    pub current_period_end: String,
    pub school_account: String,
    pub loader: AsyncLoader,
}

impl From<UserResponse> for EditableUser {
    fn from(user: UserResponse) -> Self {
        let subscription = match user.plan_type {
            Some(plan_type) => {
                let due = {
                    let due = user
                        .amount_due_in_cents
                        .map(|due| due.inner())
                        .unwrap_or_default();
                    if due > 0 {
                        format!(", Due: {}", (due as f32) / 100.0)
                    } else {
                        Default::default()
                    }
                };

                format!(
                    "{plan_type} ({}{due})",
                    user.subscription_status
                        .map(|status| status.to_string())
                        .unwrap_or_default()
                )
            }
            None => Default::default(),
        };

        let school_account = match user.school_name {
            Some(school_name) => {
                let user_type = if user.is_admin.unwrap_or_default() {
                    "Admin".to_string()
                } else {
                    "User".to_string()
                };

                format!("{school_name} ({user_type})")
            }
            None => Default::default(),
        };

        let current_period_end = match user.current_period_end {
            Some(current_period_end) => {
                let trial = user.is_trial.map_or(Default::default(), |is_trial| {
                    if is_trial {
                        "(Trial)".to_string()
                    } else {
                        Default::default()
                    }
                });

                format!("{}{trial}", current_period_end.date_naive().to_string())
            }
            None => Default::default(),
        };

        Self {
            id: user.id,
            username: Mutable::new(user.username),
            first_name: Mutable::new(user.given_name),
            last_name: Mutable::new(user.family_name),
            organization: Mutable::new(user.organization.unwrap_or_default()),
            signup_date: Mutable::new(user.created_at.to_string()),
            language: Mutable::new(user.language),
            city: Mutable::new(user.city.unwrap_or_default()),
            state: Mutable::new(user.state.unwrap_or_default()),
            country: Mutable::new(user.country.unwrap_or_default()),
            email: Mutable::new(user.email),
            badge: Mutable::new(user.badge),
            subscription,
            current_period_end,
            school_account,
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
