use super::state::*;
use components::image::tag::ImageTag;
use dominator::clone;
use shared::{
    api::{endpoints, ApiEndpoint},
    domain::image::tag::*,
    error::*,
};
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::prelude::*;

//index is id. See https://github.com/ji-devs/ji-cloud/pull/1082
impl ImageTags {
    pub fn load_init(state: Rc<Self>) {
        state.loader.load(clone!(state => async move {
            match api_with_auth::<ImageTagListResponse, EmptyError, _>(endpoints::image::tag::List::PATH, endpoints::image::tag::List::METHOD, None::<()>).await {
                Ok(resp) => {
                    state.list.lock_mut().replace_cloned(
                        resp.image_tags
                            .into_iter()
                            .map(Rc::new)
                            .collect()
                    );
                },
                Err(_) => {
                    todo!();
                }
            }
        }));
    }

    //This syncs the defined ImageTags in components with the DB
    //it does *not* update anything that was already tagged
    //so be very careful with deletions (prefer deprecation instead!)
    //
    //it's not the most efficient, e.g. adding new tags will be re-searched (in the local vec)
    //but no biggie
    pub fn sync_all(state: Rc<Self>) {
        state.loader.load(clone!(state => async move {
            let mut curr_list = state.list.lock_mut();

            for tag in ImageTag::iter() {
                if let Some((index, curr)) = curr_list.iter().enumerate().find(|(_index, curr)| curr.index.0 == tag.as_index()) {
                    if curr.display_name != tag.display_name() {
                        let req = ImageTagUpdateRequest {
                            display_name: Some(tag.display_name().to_string()),
                            index: None
                        };
                        let path = endpoints::image::tag::Update::PATH.replace("{index}", &tag.as_index().to_string());
                        let _ = api_with_auth_empty::<EmptyError, _>(&path, endpoints::image::tag::Update::METHOD, Some(req)).await.unwrap_ji();

                        let curr_index = curr.index;
                        curr_list.set_cloned(index, Rc::new(ImageTagResponse {
                            index: curr_index,
                            display_name: tag.display_name().to_string(),
                        }));
                    }
                } else {
                    let req = ImageTagCreateRequest {
                        display_name: tag.display_name().to_string()
                    };

                    let path = endpoints::image::tag::Create::PATH.replace("{index}", &tag.as_index().to_string());

                    let resp = api_with_auth::<ImageTagResponse, EmptyError, _>(&path, endpoints::image::tag::Create::METHOD, Some(req)).await.unwrap_ji();

                    curr_list.push_cloned(Rc::new(resp));
                }
            }

            let mut to_remove:Vec<i16> = Vec::new();

            curr_list
                .retain(|curr| {
                    let flag = ImageTag::iter().any(|tag| tag.as_index() == curr.index.0);

                    if !flag {
                        to_remove.push(curr.index.0);
                    }
                    flag
                });

            for index in to_remove.iter() {
                let path = endpoints::image::tag::Delete::PATH.replace("{index}", &index.to_string());
                let _ = api_with_auth_empty::<EmptyError, _>(&path, endpoints::image::tag::Delete::METHOD, None::<()>).await.unwrap_ji();
            }
        }));
    }
}
