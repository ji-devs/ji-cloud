use actix_http::body::BoxBody;
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
use std::{cell::RefCell, convert::TryFrom, rc::Rc};
use std::{collections::HashMap, fmt};
use tracing::instrument;
use uuid::Uuid;

#[derive(Debug)]
pub struct RawCategory {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub index: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_scopes: Vec<i16>,
}

#[derive(Debug, Clone)]
struct CategoryNode {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub user_scopes: Vec<i16>,
    pub children: Vec<Rc<RefCell<CategoryNode>>>,
}

impl From<CategoryNode> for Category {
    fn from(category_node: CategoryNode) -> Self {
        Self {
            id: CategoryId(category_node.id),
            name: category_node.name,
            created_at: category_node.created_at,
            updated_at: category_node.updated_at,
            user_scopes: {
                category_node
                    .user_scopes
                    .into_iter()
                    .map(|x| UserScope::try_from(x).expect("detected an invalid user scope"))
                    .collect()
            },
            children: {
                category_node
                    .children
                    .iter()
                    .map(|category| {
                        let category: CategoryNode = (**category).borrow().clone();
                        category.into()
                    })
                    .collect()
            },
        }
    }
}

#[instrument(skip_all)]
pub fn build_tree(categories: Vec<RawCategory>) -> Vec<Category> {
    let mut nodes: Vec<Rc<RefCell<CategoryNode>>> = Vec::new();
    let mut lookup: HashMap<Uuid, Rc<RefCell<CategoryNode>>> = HashMap::new();

    // Now we know this category exists in the lookup table
    for raw in categories.iter() {
        lookup.insert(
            raw.id.clone(),
            Rc::new(RefCell::new(CategoryNode {
                id: raw.id.clone(),
                name: raw.name.clone(),
                created_at: raw.created_at,
                updated_at: raw.updated_at,
                user_scopes: raw.user_scopes.clone(),
                children: Vec::new(),
            })),
        );
    }

    for raw in categories.iter() {
        let current_node = lookup
            .get(&raw.id)
            .expect("Category wasn't added to lookup map");
        match raw.parent_id {
            None => {
                // This is a root category, add it to the nodes list
                nodes.push(current_node.clone());
            }
            Some(parent_id) => {
                let parent_node = lookup
                    .get(&parent_id)
                    .expect("Parent caregory wasn't added to lookup map");
                let mut parent_node = (**parent_node).borrow_mut();
                parent_node.children.push(current_node.clone());
            }
        }
    }

    nodes
        .iter()
        .map(|node| {
            let category: CategoryNode = (**node).borrow().clone();
            let category: Category = category.into();
            category
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
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<BoxBody> {
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
