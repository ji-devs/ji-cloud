use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::settings::SETTINGS;
use std::fmt;
use crate::components::{
    module_page::ModulePageKind,
    image::data::*,
};

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
/*
import moduleEditPagePlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";
import modulePlayPage from "@templates/module/_common/module-play-page.html";
*/
const MODULE_EDIT_PAGE_PLAIN:&'static str = "module-edit-page-plain";
const MODULE_EDIT_PAGE_RESIZE:&'static str = "module-edit-page-resize";
const MODULE_PLAY_PAGE:&'static str = "module-play-page";

const IMAGE_SEARCH_WIDGET:&'static str = "image-search-widget";
const IMAGE_SEARCH_RESULT_THUMBNAIL:&'static str = "image-search-recent-thumbnail";
const IMAGE_SEARCH_RECENT_THUMBNAIL:&'static str = "image-search-result-thumbnail";

const IMAGE_TRANSFORM:&'static str = "image-transform";

pub fn module_page(kind:ModulePageKind) -> HtmlElement {
    match kind {
        ModulePageKind::EditPlain => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(MODULE_EDIT_PAGE_PLAIN))
        },
        ModulePageKind::EditResize => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(MODULE_EDIT_PAGE_RESIZE))
        },
        ModulePageKind::Play => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(MODULE_PLAY_PAGE))
        }
    }
}

pub fn image_search_widget() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGE_SEARCH_WIDGET))
}

pub fn image_search_result_thumbnail(img:&MetaImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_SEARCH_RESULT_THUMBNAIL, &html_map!{
        "name" => &img.meta.name,
        "src" => &img.thumbnail_src(),
    }).unwrap_throw())
}

pub fn image_search_recent_thumbnail(img:&MetaImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_SEARCH_RECENT_THUMBNAIL, &html_map!{
        "src" => &img.thumbnail_src(),
    }).unwrap_throw())
}

pub fn image_transform(img:&SimpleImage) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_TRANSFORM, &html_map!{
        "src" => &img.full_src(),
    }).unwrap_throw())
}

pub struct Templates {
    pub cache: TemplateCache<'static>
}

impl fmt::Debug for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        f.debug_list()
            .entries(self.cache.templates.keys())
         .finish()
    }
}

macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../.template_output/", $e)
    } 
}
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (MODULE_EDIT_PAGE_PLAIN, get_template_str(include_str!(
                template_path!("module/_common/module-edit-page-plain.html")
            ))),
            (MODULE_EDIT_PAGE_RESIZE, get_template_str(include_str!(
                template_path!("module/_common/module-edit-page-resize.html")
            ))),
            (MODULE_PLAY_PAGE, get_template_str(include_str!(
                template_path!("module/_common/module-play-page.html")
            ))),
            (IMAGE_SEARCH_WIDGET, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/widget.html")
            ))),
            (IMAGE_SEARCH_RESULT_THUMBNAIL, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/result-thumbnail.html")
            ))),
            (IMAGE_SEARCH_RECENT_THUMBNAIL, get_template_str(include_str!(
                template_path!("_common/widgets/image-search/recent-thumbnail.html")
            ))),
            (IMAGE_TRANSFORM, get_template_str(include_str!(
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
