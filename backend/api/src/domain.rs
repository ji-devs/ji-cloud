
use shared::domain::category::{Category, CategoryId};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct RawCategory {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub index: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub image_count: i64,
    pub jig_count: i64,
}

pub(crate) fn build_tree(mut categories: Vec<RawCategory>) -> Vec<Category> {
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
