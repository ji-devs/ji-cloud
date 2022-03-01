pub use shared::domain::jig::module::body::ThemeId;
use shared::domain::meta::{AgeRange, AgeRangeId};

const STR_ALL_AGES: &str = "All Ages";
const STR_DASH: &str = "-";

pub trait AgeRangeVecExt {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String;
}

impl AgeRangeVecExt for Vec<AgeRange> {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String {
        if selected.len() == self.len() || selected.len() == 0 {
            STR_ALL_AGES.to_string()
        } else if selected.len() == 1 {
            let age = self.iter().find(|age| selected.contains(&age.id));

            if let Some(age) = age {
                let age_text = age
                    .short_display_name
                    .as_ref()
                    .unwrap_or(&age.display_name)
                    .to_string();

                age_text
            } else {
                String::new()
            }
        } else {
            let first_age = self.iter().find(|age| selected.contains(&age.id));
            let last_age = self.iter().rev().find(|age| selected.contains(&age.id));

            if let (Some(first_age), Some(last_age)) = (first_age, last_age) {
                let first_age_text = first_age
                    .short_display_name
                    .as_ref()
                    .unwrap_or(&first_age.display_name);

                let last_age_text = last_age
                    .short_display_name
                    .as_ref()
                    .unwrap_or(&last_age.display_name);

                let mut age_text = String::new();

                age_text.push_str(first_age_text);
                age_text.push_str(STR_DASH);
                age_text.push_str(last_age_text);

                age_text
            } else {
                String::new()
            }
        }
    }
}
