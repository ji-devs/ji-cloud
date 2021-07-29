use shared::domain::{
    image::ImageId,
    meta::{AffiliationId, AgeRangeId, MetadataResponse}
};

#[derive(Debug)]
pub struct MetaInfo {
    pub affiliation_all_id: AffiliationId,
    pub age_ranges_all_id: AgeRangeId,
    pub affiliations_for_all: Vec<AffiliationId>,
    pub age_ranges_for_all: Vec<AgeRangeId>,
}

impl From<MetadataResponse> for MetaInfo {
    fn from(resp:MetadataResponse) -> Self {
        fn match_all(x:&str) -> bool {
            let all_list = [
                "All",
                "All ages",
            ];

            all_list.contains(&x)
        }

        //log::info!("{:#?}", resp);

        let hit = resp.affiliations.iter().find(|x| match_all(&x.display_name)).expect("Couldn't find all affiliations id!");
        let affiliation_all_id = hit.id;

        let hit = resp.age_ranges.iter().find(|x| match_all(&x.display_name)).expect("couldn't find all age ranges id!");
        let age_ranges_all_id = hit.id;

        Self {
            affiliation_all_id,
            age_ranges_all_id,
            affiliations_for_all: resp
                .affiliations
                .iter()
                .map(|x| x.id)
                .filter(|x| *x != affiliation_all_id)
                .collect(),
            age_ranges_for_all: resp
                .age_ranges
                .iter()
                .map(|x| x.id)
                .filter(|x| *x != age_ranges_all_id)
                .collect(),
                 
        }
    }
}

pub struct ImageInfo {
    pub id: ImageId,
    pub has_all_affiliations: bool,
    pub has_all_age_ranges: bool
}
