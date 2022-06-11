use shared::domain::meta::{AgeRange, AgeRangeId};
pub use shared::domain::module::body::ThemeId;

use crate::prelude::UnwrapJiExt;

const STR_ALL_AGES: &str = "All Ages";

pub trait AgeRangeVecExt {
    fn range(&self, selected: &[AgeRangeId]) -> (String, Option<String>);
}

impl AgeRangeVecExt for Vec<AgeRange> {
    fn range(&self, selected: &[AgeRangeId]) -> (String, Option<String>) {
        if selected.len() == self.len() || selected.is_empty() {
            (STR_ALL_AGES.to_string(), None)
        } else if selected.len() == 1 {
            (get_age_text(self, selected, false), None)
        } else {
            let first_age_text = get_age_text(self, selected, false);
            let last_age_text = get_age_text(self, selected, true);

            (first_age_text, Some(last_age_text))
        }
    }
}

fn get_age_text(ages: &[AgeRange], selected: &[AgeRangeId], get_last: bool) -> String {
    match get_last {
        false => ages
            .iter()
            .find(|age| selected.contains(&age.id))
            .map(|age_range| {
                age_range
                    .short_display_name
                    .as_ref()
                    .unwrap_or(&age_range.display_name)
                    .to_string()
            })
            .unwrap_ji(),
        true => ages
            .iter()
            .rev()
            .find(|age| selected.contains(&age.id))
            .map(|age_range| {
                age_range
                    .short_display_name
                    .as_ref()
                    .unwrap_or(&age_range.display_name)
                    .to_string()
            })
            .unwrap_ji(),
    }
}
