use super::sections::common::categories::MutableCategory;
use super::state::*;
use components::image::tag::ImageTag;
use components::image::upload::upload_image;
use dominator::clone;
use shared::{
    api::endpoints,
    domain::{category::*, image::tag::*, image::*, meta::*, Publish},
    media::MediaLibrary,
};
use std::cell::RefCell;
use std::rc::Rc;
use strum::IntoEnumIterator;
use utils::prelude::*;
use web_sys::File;

pub fn load_initial(
    state: Rc<State>,
) -> Rc<
    RefCell<
        Option<(
            Rc<MutableImage>,
            Rc<Vec<Rc<MutableCategory>>>,
            Rc<MetadataResponse>,
        )>,
    >,
> {
    let ret = Rc::new(RefCell::new(None));

    state.loader.load(clone!(state, ret => async move {

        let cat_req = GetCategoryRequest{ ids: Vec::new(), scope: Some( CategoryTreeScope::Descendants) } ;
        match (
            endpoints::image::Get::api_with_auth(ImageGetPath(state.id.clone()), None).await,
            endpoints::category::Get::api_with_auth(GetCategoryPath(), Some(cat_req)).await,
            endpoints::meta::Get::api_no_auth(GetMetadataPath(), None).await,
            endpoints::image::tag::List::api_with_auth(ImageTagListPath(), None).await
        ) {
            (Ok(img_resp), Ok(cat_resp), Ok(meta_resp), Ok(tag_list_resp)) => {

                let image:Rc<MutableImage> = Rc::new(img_resp.metadata.into());

                let categories:Rc<Vec<Rc<MutableCategory>>> = Rc::new(cat_resp
                    .categories
                    .into_iter()
                    .map(|cat| Rc::new(cat.into()))
                    .collect()
                );

                let meta:Rc<MetadataResponse> = Rc::new(meta_resp);

                //Sanity check the tag list
                for db_tag in tag_list_resp.image_tags.iter() {
                    let _ = ImageTag::iter()
                        .find(|tag| tag.as_index() == db_tag.index.0)
                        .expect_ji(&format!("Image tag {} was in DB but not Rust!", db_tag.display_name));
                }

                for rust_tag in ImageTag::iter() {
                    let _ = tag_list_resp.image_tags.iter()
                        .find(|db_tag| db_tag.index.0 == rust_tag.as_index())
                        .expect_ji(&format!("Image tag {} was in Rust but not Db!", rust_tag.display_name()));
                }

                *ret.borrow_mut() = Some((image, categories, meta));
                state.loaded.set(true);
            },
            _errors => {
                log::error!("error loading initial data!")
            }
        }
    }));

    ret
}

pub fn save(state: Rc<State>, req: ImageUpdateRequest) {
    state.loader.load(clone!(state => async move {

        match endpoints::image::UpdateMetadata::api_with_auth(ImageUpdatePath(state.id.clone()), Some(req)).await {
            Ok(_) => {
            },
            Err(_err) => {
                log::error!("couldn't save!");
            },
        }
    }));
}

pub fn on_file(state: Rc<State>, image: Rc<MutableImage>, file: File) {
    state.loader.load(clone!(state => async move {

        match upload_image(state.id, MediaLibrary::Global, &file, None).await {
            Ok(_) => {
                //Trigger a re-render.
                //To debug: this shouldn't be necessary, but it temp fixes!
                //TimeoutFuture::new(5_000).await;
                image.id.replace_with(|id| *id);
            },
            Err(err) => {
                if err.is_abort() {
                    log::info!("aborted!");
                } else {
                    log::error!("got error! {:?}", err);
                }
            }
        }
    }))
}

pub fn toggle_premium(state: Rc<State>, image: Rc<MutableImage>, is_premium: bool) {
    image.is_premium.set_neq(is_premium);
    save(
        state,
        ImageUpdateRequest {
            is_premium: Some(is_premium),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn change_name(state: Rc<State>, image: Rc<MutableImage>, name: String) {
    image.name.set_neq(name.clone());
    save(
        state,
        ImageUpdateRequest {
            name: Some(name),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn change_description(state: Rc<State>, image: Rc<MutableImage>, description: String) {
    image.description.set_neq(description.clone());
    save(
        state,
        ImageUpdateRequest {
            description: Some(description),
            ..ImageUpdateRequest::default()
        },
    );
}

pub fn delete(state: Rc<State>) {
    state.loader.load(clone!(state => async move {
        match endpoints::image::Delete::api_with_auth(ImageDeletePath(state.id), None).await {
            Ok(_) => {
                let route:String = Route::Admin(AdminRoute::ImageSearch(None)).into();
                dominator::routing::go_to_url(&route);
            },
            Err(_err) => {
                log::error!("couldn't save!");
            },
        }
    }));
}

pub fn publish(state: Rc<State>) {
    save(
        state,
        ImageUpdateRequest {
            publish_at: Some(Some(Publish::now())),
            ..ImageUpdateRequest::default()
        },
    );
}
