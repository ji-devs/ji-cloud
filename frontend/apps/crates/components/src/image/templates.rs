use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;
use super::data::*;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

pub struct Templates {
    pub cache: TemplateCache<'static>
}

macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../.template_output/", $e)
    } 
}
const SEARCH_WIDGET:&'static str = "image-search-widget";
const SEARCH_RESULT_THUMBNAIL:&'static str = "image-search-recent-thumbnail";
const SEARCH_RECENT_THUMBNAIL:&'static str = "image-search-result-thumbnail";

const TRANSFORM:&'static str = "image-transform";

pub fn search_widget() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SEARCH_WIDGET))
}

pub fn search_result_thumbnail(img:&MetaImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(SEARCH_RESULT_THUMBNAIL, &html_map!{
        "name" => &img.meta.name,
        "src" => &img.thumbnail_src(),
    }).unwrap_throw())
}

pub fn search_recent_thumbnail(img:&MetaImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(SEARCH_RECENT_THUMBNAIL, &html_map!{
        "src" => &img.thumbnail_src(),
    }).unwrap_throw())
}

pub fn transform(img:&SimpleImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(TRANSFORM, &html_map!{
        "src" => &img.full_src(),
    }).unwrap_throw())
}

impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (SEARCH_WIDGET, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/widget.html")
            ))),
            (SEARCH_RESULT_THUMBNAIL, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/result-thumbnail.html")
            ))),
            (SEARCH_RECENT_THUMBNAIL, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/recent-thumbnail.html")
            ))),
            (TRANSFORM, get_template_str(include_str!(
                template_path!("_common/image/image-transform.html")
            ))),
        ]);

        Self { cache }
    }

}

//replace {{MEDIA_UI}} in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
