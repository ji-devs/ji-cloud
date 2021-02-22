use chrono::{DateTime, Utc};
use shared::domain::category::{Category, CategoryId};
use std::collections::HashMap;
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
    let indecies = match parent_to_id_index.get(&seed_id) {
        Some(indecies) => indecies,
        None => return Vec::new(),
    };

    indecies
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
