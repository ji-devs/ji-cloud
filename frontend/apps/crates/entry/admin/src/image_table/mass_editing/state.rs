use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use futures_signals::signal::Mutable;
use shared::domain::meta::{AffiliationId, AgeRangeId, ImageStyleId, ImageTagIndex};

use crate::image_table::ImageTable;

pub struct MassEditing {
    pub(super) images_state: Rc<ImageTable>,
    pub(super) mode: Mutable<Mode>,
    pub(super) styles: Mutable<HashSet<ImageStyleId>>,
    pub(super) tags: Mutable<HashSet<ImageTagIndex>>,
    pub(super) ages: Mutable<HashSet<AgeRangeId>>,
    pub(super) affiliations: Mutable<HashSet<AffiliationId>>,
    // pub(super) categories: Mutable<HashSet<CategoryId>>,
}

impl MassEditing {
    pub fn new(images_state: &Rc<ImageTable>) -> Rc<Self> {
        Rc::new(Self {
            images_state: Rc::clone(&images_state),
            mode: Mutable::new(Mode::Add),
            styles: Default::default(),
            tags: Default::default(),
            ages: Default::default(),
            affiliations: Default::default(),
        })
    }

    pub(super) fn calculate_selected_counts(&self) -> Rc<SelectedCounts> {
        fn num_to_selected_count_map<T: std::hash::Hash + Eq>(
            map: HashMap<T, u32>,
            selected_count: u32,
        ) -> HashMap<T, SelectedCount> {
            map.into_iter()
                .map(|(id, count)| {
                    let val = if count == selected_count as u32 {
                        SelectedCount::All
                    } else if count == 0 {
                        SelectedCount::None
                    } else {
                        SelectedCount::Count(count)
                    };
                    (id, val)
                })
                .collect()
        }

        let images = self.images_state.images.lock_ref();
        let selected_images = self.images_state.selected_images.lock_ref();

        let mut styles: HashMap<ImageStyleId, u32> = HashMap::new();
        for image in images.iter() {
            if selected_images.contains(&image.id) {
                for style in image.styles.lock_ref().iter() {
                    *styles.entry(*style).or_default() += 1;
                }
            }
        }
        let mut tags: HashMap<ImageTagIndex, u32> = HashMap::new();
        for image in images.iter() {
            if selected_images.contains(&image.id) {
                for tag in image.tags.lock_ref().iter() {
                    *tags.entry(*tag).or_default() += 1;
                }
            }
        }
        let mut ages: HashMap<AgeRangeId, u32> = HashMap::new();
        for image in images.iter() {
            if selected_images.contains(&image.id) {
                for age in image.age_ranges.lock_ref().iter() {
                    *ages.entry(*age).or_default() += 1;
                }
            }
        }
        let mut affiliations: HashMap<AffiliationId, u32> = HashMap::new();
        for image in images.iter() {
            if selected_images.contains(&image.id) {
                for affiliation in image.affiliations.lock_ref().iter() {
                    *affiliations.entry(*affiliation).or_default() += 1;
                }
            }
        }

        let selected_count = selected_images.len() as u32;
        Rc::new(SelectedCounts {
            styles: num_to_selected_count_map(styles, selected_count),
            tags: num_to_selected_count_map(tags, selected_count),
            ages: num_to_selected_count_map(ages, selected_count),
            affiliations: num_to_selected_count_map(affiliations, selected_count),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Mode {
    Add,
    Remove,
}

#[derive(Clone, PartialEq, Eq, Default)]
pub(super) struct SelectedCounts {
    pub styles: HashMap<ImageStyleId, SelectedCount>,
    pub tags: HashMap<ImageTagIndex, SelectedCount>,
    pub ages: HashMap<AgeRangeId, SelectedCount>,
    pub affiliations: HashMap<AffiliationId, SelectedCount>,
    // pub categories: HashMap<CategoryId, SelectedCount>,
}

impl SelectedCounts {
    pub fn get_styles_string(&self, id: &ImageStyleId) -> String {
        self.styles
            .get(id)
            .map(|style| style.to_string())
            .unwrap_or_else(|| SelectedCount::None.to_string())
    }
    pub fn get_tags_string(&self, id: &ImageTagIndex) -> String {
        self.tags
            .get(id)
            .map(|tag| tag.to_string())
            .unwrap_or_else(|| SelectedCount::None.to_string())
    }
    pub fn get_ages_string(&self, id: &AgeRangeId) -> String {
        self.ages
            .get(id)
            .map(|age| age.to_string())
            .unwrap_or_else(|| SelectedCount::None.to_string())
    }
    pub fn get_affiliations_string(&self, id: &AffiliationId) -> String {
        self.affiliations
            .get(id)
            .map(|affiliation| affiliation.to_string())
            .unwrap_or_else(|| SelectedCount::None.to_string())
    }
    // pub fn get_categories_string(&self, id: &CategoryId) -> String {
    //     self.categories.get(id).map(|category| category.to_string()).unwrap_or_default()
    // }
}

#[derive(Clone, PartialEq, Eq, Default)]
pub(super) enum SelectedCount {
    #[default]
    None,
    Count(u32),
    All,
}

impl SelectedCount {
    pub fn to_string(&self) -> String {
        match self {
            SelectedCount::None => String::from("(None)"),
            SelectedCount::Count(count) => format!("({})", count.to_string()),
            SelectedCount::All => String::from("(All)"),
        }
    }
}
