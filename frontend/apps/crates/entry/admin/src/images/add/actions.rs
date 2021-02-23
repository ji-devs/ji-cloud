use shared::domain::image::ImageKind;
use super::state::*;
use std::rc::Rc;
use web_sys::File;

pub fn on_change(state: Rc<State>, value: String) {
    match value.as_ref() {
        "sticker" => {
            *state.kind.borrow_mut() = ImageKind::Sticker;
        },
        "canvas" => {
            *state.kind.borrow_mut() = ImageKind::Canvas;
        },
        _ => {
            log::info!("unknown value [{}]", value);
        }
    }
}

pub fn on_file(state: Rc<State>, file: File) {
    log::info!("TODO - upload file");
/*
spawn_local(async move {
    let id = actions::create_image(file, get_image_kind()).await.unwrap_throw();
    let route:String = Route::Admin(AdminRoute::ImageEdit(id, None)).into();
    dominator::routing::go_to_url(&route);
});
*/
}
