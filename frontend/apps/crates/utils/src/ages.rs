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
            get_age_text(self, selected, false)
        } else {
            let first_age_text = get_age_text(self, selected, false);
            let last_age_text = get_age_text(self, selected, true);
            let mut age_text = String::new();
            if first_age_text != "" && last_age_text != "" {
                age_text.push_str(&first_age_text);
                age_text.push_str(STR_DASH);
                age_text.push_str(&last_age_text);
            }
            age_text
        }
    }
}

fn get_age_text(ages: &Vec<AgeRange>, selected: &Vec<AgeRangeId>, get_last: bool) -> String {
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
            .unwrap_or(String::new()),
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
            .unwrap_or(String::new()),
    }
}
