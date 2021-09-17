use actix_web::{
    cookie::{Cookie, SameSite},
    HttpResponse, Responder,
};
use chrono::{DateTime, Utc};
use http::StatusCode;
use shared::domain::user::UserScope;
use shared::domain::{
    category::{Category, CategoryId},
    session::AUTH_COOKIE_NAME,
};
use std::convert::TryFrom;
use std::{collections::HashMap, fmt};
use uuid::Uuid;

#[derive(Debug)]
pub struct RawCategory {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub index: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub image_count: i64,
    pub jig_count: i64,
    pub user_scopes: Vec<i16>,
}

pub fn build_tree(mut categories: Vec<RawCategory>) -> Vec<Category> {
    let mut parent_to_id_index: HashMap<Option<Uuid>, Vec<usize>> = HashMap::new();

    for (idx, category) in categories.iter().enumerate() {
        parent_to_id_index
            .entry(category.parent_id)
            .and_modify(|it| it.push(idx))
            .or_insert_with(|| vec![idx]);
    }

    parent_to_id_index
        .values_mut()
        .for_each(|it| it.sort_unstable_by_key(|v| categories[*v].index));

    build_tree_recursive(&parent_to_id_index, &mut categories, None)
}

fn build_tree_recursive(
    parent_to_id_index: &HashMap<Option<Uuid>, Vec<usize>>,
    categories: &mut Vec<RawCategory>,
    seed_id: Option<Uuid>,
) -> Vec<Category> {
    let indices = match parent_to_id_index.get(&seed_id) {
        Some(indecies) => indecies,
        None => return Vec::new(),
    };

    indices
        .iter()
        .copied()
        .map(|category_index| {
            let children = build_tree_recursive(
                parent_to_id_index,
                categories,
                Some(categories[category_index].id),
            );

            let raw: &mut RawCategory = &mut categories[category_index];

            Category {
                id: CategoryId(raw.id),
                name: raw.name.clone(),
                created_at: raw.created_at,
                updated_at: raw.updated_at,
                image_count: raw.image_count as u64,
                jig_count: raw.jig_count as u64,
                user_scopes: {
                    raw.user_scopes
                        .clone()
                        .into_iter()
                        .map(|x| UserScope::try_from(x).expect("detected an invalid user scope"))
                        .collect()
                },
                children,
            }
        })
        .collect()
}

#[derive(Debug, Copy, Clone, sqlx::Type)]
#[repr(u16)]
pub enum RegistrationStatus {
    /// The user was just registered ([`POST /v1/user`](shared::api::endpoints::user::Register))
    ///
    /// This state is *skipped* via ouath (they're automatically [`Validated`](Self::Validated))
    /// In this state the user is allowed to:
    /// * Request a new verification email
    /// * Request a password reset(?)
    New = 0,

    /// The user has gone through OAuth and created a new user, or has finished email verification.
    ///
    /// In this state the user is allowed to:
    /// * Request a password reset
    /// * Complete registration via creating their profile
    /// * Delete their account
    Validated = 1,

    /// The user has *completely* finished signup, and their profile has been completed.
    ///
    /// In this state the user is allowed to do anything that their scopes allow.
    /// All users can:
    /// * Change their email (not implemented nor currently on the roadmap)
    /// * Change their password if logged in, reset their password if not.
    /// * Access the API
    /// * Change their profile details (not currently implemented)
    /// * Delete their account
    Complete = 2,
}

#[derive(Debug)]
pub struct NoContentClearAuth;

impl fmt::Display for NoContentClearAuth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("No Content")
    }
}

impl Responder for NoContentClearAuth {
    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse {
        let mut cookie = Cookie::named(AUTH_COOKIE_NAME);
        cookie.set_max_age(time::Duration::seconds(0));
        cookie.set_http_only(true);
        cookie.set_same_site(SameSite::Strict);

        actix_web::HttpResponse::build(StatusCode::NO_CONTENT)
            .content_type("application/octet-stream")
            .cookie(cookie)
            .finish()
    }
}
