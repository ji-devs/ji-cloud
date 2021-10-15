pub use shared::domain::jig::module::body::ThemeId;
use shared::domain::meta::{AgeRange, AgeRangeId};

pub trait AgeRangeVecExt {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String;
}

impl AgeRangeVecExt for Vec<AgeRange> {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String {
        let first_age = self.iter().find(|age| selected.contains(&age.id));

        let mut result = String::new();
        if let Some(first_age) = first_age {
            result = first_age.display_name.clone();
        };

        if selected.len() > 1 {
            let additional_ages = selected.len() - 1;
            result.push_str(" +");
            result.push_str(&additional_ages.to_string());
        }

        result
    }
}
