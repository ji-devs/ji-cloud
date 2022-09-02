use super::super::state::Gallery;
use shared::{
    api::endpoints::{self},
    domain::{
        asset::{Asset, DraftOrLive, UserOrMe},
        course::{
            CourseBrowsePath, CourseBrowseQuery, CourseCreatePath, CourseCreateRequest,
            CourseDeletePath, CourseId, CourseSearchPath, CourseSearchQuery,
        },
        module::{ModuleBody, ModuleCreatePath, ModuleCreateRequest, ModuleKind},
    },
};
use std::rc::Rc;
use utils::prelude::*;

pub async fn load_courses(
    state: &Rc<Gallery>,
    is_published: Option<bool>,
) -> Result<(Vec<Asset>, u64), ()> {
    let req = CourseBrowseQuery {
        page: Some(*state.next_page.lock_ref()),
        is_published,
        author_id: Some(UserOrMe::Me),
        draft_or_live: Some(DraftOrLive::Draft),
        ..Default::default()
    };

    endpoints::course::Browse::api_with_auth(CourseBrowsePath(), Some(req))
        .await
        .map(|res| {
            let assets = res
                .courses
                .into_iter()
                .map(|course| course.into())
                .collect();
            (assets, res.total_course_count)
        })
        .map_err(|_| ())
}

pub async fn search_courses(q: String, is_published: Option<bool>) -> Result<Vec<Asset>, ()> {
    let req = CourseSearchQuery {
        q,
        is_published,
        author_id: Some(UserOrMe::Me),
        ..Default::default()
    };

    endpoints::course::Search::api_with_auth(CourseSearchPath(), Some(req))
        .await
        .map(|resp| {
            resp.courses
                .into_iter()
                .map(|course| course.into())
                .collect()
        })
        .map_err(|_| ())
}

pub async fn create_course() {
    let req = CourseCreateRequest {
        ..Default::default()
    };

    match endpoints::course::Create::api_with_auth(CourseCreatePath(), Some(req)).await {
        Ok(resp) => {
            add_cover(&resp.id).await;
            let url = Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                resp.id,
                CourseEditRoute::Landing,
            )))
            .to_string();
            dominator::routing::go_to_url(&url);
        }
        Err(_) => todo!(""),
    }
}

async fn add_cover(course_id: &CourseId) {
    let req = ModuleCreateRequest {
        body: ModuleBody::new(ModuleKind::ResourceCover),
        parent_id: (*course_id).into(),
    };

    // let path = endpoints::module::Create::PATH.replace("{id}", &jig_id.0.to_string());

    match endpoints::module::Create::api_with_auth(ModuleCreatePath(), Some(req)).await {
        Ok(_) => {}
        Err(_) => {
            todo!()
        }
    }
}

pub async fn copy_course(_course_id: CourseId) -> Result<Asset, ()> {
    todo!()
    // let path = endpoints::course::Clone::PATH.replace("{id}", &course_id.0.to_string());

    // match api_with_auth::<CreateResponse<CourseId>, EmptyError, ()>(
    //     &path,
    //     endpoints::course::Clone::METHOD,
    //     None,
    // )
    // .await
    // {
    //     Ok(resp) => {
    //         let path = endpoints::course::GetDraft::PATH.replace("{id}", &resp.id.0.to_string());
    //         api_with_auth::<CourseResponse, EmptyError, ()>(
    //             &path,
    //             endpoints::course::GetDraft::METHOD,
    //             None,
    //         )
    //         .await
    //         .map(|resp| {
    //             let asset: Asset = resp.into();
    //             asset
    //         })
    //         .map_err(|_| ())
    //     }
    //     Err(_) => Err(()),
    // }
}

pub async fn delete_course(course_id: CourseId) -> anyhow::Result<()> {
    endpoints::course::Delete::api_with_auth_empty(CourseDeletePath(course_id), None)
        .await
        .map(|_| ())
}
