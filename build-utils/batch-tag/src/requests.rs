use shared::{api::{ApiEndpoint, endpoints}, domain::{image::*, meta::MetadataResponse}};
use reqwest::Body;
use super::context::Context;
use super::data::{ImageInfo, MetaInfo};
use std::{
    sync::Arc
};

pub async fn get_meta(ctx: Arc<Context>) -> anyhow::Result<MetaInfo>  {
    let client = reqwest::Client::new();
   
    if(ctx.opts.verbose) {
        log::info!("[verbose] getting meta");
    }

    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), endpoints::meta::Get::PATH);
    let resp = client
        .get(&url)
        .send()
        .await?
        .error_for_status()?;

    let body: serde_json::Value = resp.json().await?;
    let body:MetadataResponse = serde_json::from_value(body)?;
    let data = MetaInfo::from(body);


    Ok(data)
}

pub async fn get_image_list(ctx: Arc<Context>, meta: &MetaInfo) -> anyhow::Result<Vec<ImageInfo>>  {
    if(ctx.opts.verbose) {
        log::info!("[verbose] getting image list");
    }
    let mut list:Vec<ImageInfo> = Vec::new();


    let mut page = 0;
    let mut total_pages = 0;
    
    let client = reqwest::Client::new();


    loop {
        let req = ImageBrowseQuery { 
            is_published: None,
            kind: None,
            page: if page == 0 { None }  else { Some(page) }
        };

        let mut query = serde_qs::to_string(&req).unwrap();

        if !query.is_empty() {
            query = format!("?{}", query);
        }

        let url = format!("{}{}{}", ctx.opts.get_remote_target().api_url(), endpoints::image::Browse::PATH, query);
        let resp = client
            .get(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .send()
            .await?
            .error_for_status()?;

        let body: serde_json::Value = resp.json().await?;
        let body:ImageBrowseResponse = serde_json::from_value(body)?;

        if page == 0 {
            total_pages = body.pages;
        }

        if(ctx.opts.verbose) {
            log::info!("[verbose] getting images for page #{} / {}", page+1, total_pages); 
        }

        ctx.report.write().await.n_total_images += body.images.len();

        for image in body.images {
            let mut has_all_affiliations = image.metadata.affiliations.iter().any(|x| *x == meta.affiliation_all_id);
            let mut has_all_age_ranges = image.metadata.age_ranges.iter().any(|x| *x == meta.age_ranges_all_id);

            if has_all_affiliations {
                let already_set = meta.affiliations_for_all.iter().all(|x| {
                    image.metadata.affiliations.iter().any(|y| x == y)
                });

                if already_set {
                    log::info!("skipping [{}] affiliations (already set)", image.metadata.id.0.to_string());

                    has_all_affiliations = false;
                }
            }
            if has_all_age_ranges {
                let already_set = meta.age_ranges_for_all.iter().all(|x| {
                    image.metadata.age_ranges.iter().any(|y| x == y)
                });

                if already_set {
                    log::info!("skipping [{}] age ranges (already set)", image.metadata.id.0.to_string());

                    has_all_age_ranges = false;
                }
            }
            if has_all_affiliations || has_all_age_ranges {
                //log::info!("{}: {} {}", image.metadata.id.0.to_string(), has_all_affiliations, has_all_age_ranges);

                list.push(ImageInfo {
                    id: image.metadata.id,
                    has_all_affiliations,
                    has_all_age_ranges
                })
            }
        }

        page += 1;

        if page >= total_pages {
            break;
        }
    }

    Ok(list)
}

pub async fn fix_image(ctx: Arc<Context>, meta: Arc<MetaInfo>, image: ImageInfo) {

    let req = ImageUpdateRequest { 
        affiliations: {
            if image.has_all_affiliations {
                let mut ids = meta.affiliations_for_all.clone();
                ids.push(meta.affiliation_all_id);
                Some(ids)
            } else {
                None
            }
        },
        age_ranges: {
            if image.has_all_age_ranges {
                let mut ids = meta.age_ranges_for_all.clone();
                ids.push(meta.age_ranges_all_id);
                Some(ids)
            } else {
                None
            }
        },
        ..ImageUpdateRequest::default()
    };

    let path = endpoints::image::UpdateMetadata::PATH.replace("{id}", &image.id.0.to_string());

    let url = format!("{}{}", ctx.opts.get_remote_target().api_url(), path);

    if !ctx.opts.dry_run {
        let client = reqwest::Client::new();
        let resp = client
            .patch(&url)
            .header("Authorization", &format!("Bearer {}", ctx.token))
            .json(&req)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();

        if !resp.status().is_success() {
            panic!("error at [{}] affiliations: {} age_ranges: {}", &image.id.0.to_string(), image.has_all_affiliations, image.has_all_age_ranges);
        }

        log::info!("fixed [{}] affiliations: {} age_ranges: {}", &image.id.0.to_string(), image.has_all_affiliations, image.has_all_age_ranges);
    } else {
        log::info!("dry run [{}] affiliations: {} age_ranges: {}", &image.id.0.to_string(), image.has_all_affiliations, image.has_all_age_ranges);
    }

    if image.has_all_affiliations {
        ctx.report.write().await.n_fixed_affiliations += 1;
    }
    if image.has_all_age_ranges {
        ctx.report.write().await.n_fixed_age_ranges += 1;
    }

}
