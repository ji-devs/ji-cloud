use shared::domain::meta::{AgeRange, AgeRangeId};
use crate::unwrap::UnwrapJiExt;
pub use shared::domain::jig::module::body::ThemeId;

pub trait AgeRangeVecExt {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String;

}

impl AgeRangeVecExt for Vec<AgeRange> {
    fn range_string(&self, selected: &Vec<AgeRangeId>) -> String {
        if self.len() == 0 || selected.len() == 0 {
            return String::new();
        }
        if selected.contains(&self[0].id) {
            return self[0].display_name.clone();
        }

        let mut first: Option<&AgeRange> = None;
        let mut last: &AgeRange = &self[0];
        for age_range in self {
            if first.is_none() && selected.contains(&age_range.id) {
                first = Some(&age_range);
            }
            if selected.contains(&age_range.id) {
                last = &age_range;
            }
        }

        let first = first.unwrap_ji().display_name.split("-");
        let first = first.collect::<Vec<_>>()[0];
        let last = {
            // the last one is "+14" so it can't be split at "-"
            if last.id == self.last().unwrap_ji().id {
                last.display_name.as_str()
            } else {
                let last = last.display_name.split("-");
                last.collect::<Vec<_>>()[1]
            }
        };
        format!("{}-{}", &first, &last)
    }
}
