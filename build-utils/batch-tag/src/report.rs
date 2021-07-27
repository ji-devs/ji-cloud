use super::data::ImageInfo;

pub struct Report {
    pub n_to_fix_affiliations:usize,
    pub n_to_fix_age_ranges:usize,
    pub n_fixed_affiliations:usize,
    pub n_fixed_age_ranges:usize
}

impl Report {
    pub fn new() -> Self {
        Self {
            n_to_fix_affiliations: 0,
            n_to_fix_age_ranges: 0,
            n_fixed_affiliations: 0,
            n_fixed_age_ranges: 0,
        }
    }

    pub fn set_from_images(&mut self, image_list:&[ImageInfo]) {
        for image in image_list.iter() {
            if image.has_all_affiliations {
                self.n_to_fix_affiliations += 1;
            }
            if image.has_all_age_ranges {
                self.n_to_fix_age_ranges += 1;
            }
        }

        log::info!("total affiliations to fix: {}", self.n_to_fix_affiliations);
        log::info!("total age ranges to fix: {}", self.n_to_fix_age_ranges);
    }

    pub fn final_log(&self) {
        log::info!("total affiliations fixed: {}", self.n_fixed_affiliations);
        log::info!("total age ranges fixed: {}", self.n_fixed_age_ranges);
    }
}
