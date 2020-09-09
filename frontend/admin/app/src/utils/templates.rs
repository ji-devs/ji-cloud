use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

const CHECKBOX:&'static str = "checkbox";
const CATEGORIES:&'static str = "categories";
const CATEGORY_MAIN_SELECTED:&'static str = "category-main-selected";
const CATEGORY_MAIN_DESELECTED:&'static str = "category-main-deselected";
const CATEGORY_SUB:&'static str = "category-sub";
const CATEGORY_LABEL_DISPLAY:&'static str = "category-label-display";
const CATEGORY_LABEL_INPUT:&'static str = "category-label-input";
const CATEGORY_MENU:&'static str = "category-menu";

const IMAGES_PAGE:&'static str = "images-page";
const IMAGE_ADD:&'static str = "image-add";
const IMAGE_EDIT:&'static str = "image-edit";
const IMAGE_EDIT_META:&'static str = "image-edit-meta";
const IMAGE_EDIT_CATEGORIES:&'static str = "image-edit-categories";

pub fn categories() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORIES))
}
pub fn category_main(id:&str, selected:bool) -> HtmlElement {
    if selected {
        TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_MAIN_SELECTED, &html_map!(
            "id" => id,
        )).unwrap())
    } else {
        TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_MAIN_DESELECTED, &html_map!(
            "id" => id,
        )).unwrap())
    }
}

pub fn category_sub(id:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(CATEGORY_SUB, &html_map!(
        "id" => id,
    )).unwrap())
}
pub fn category_label_input() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORY_LABEL_INPUT))
}
pub fn category_label_display() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORY_LABEL_DISPLAY))
}
pub fn category_menu() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORY_MENU))
}

pub fn images_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGES_PAGE))
}
pub fn image_add() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGE_ADD))
}
pub fn image_edit() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGE_EDIT))
}
pub fn image_edit_meta() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGE_EDIT_META))
}
pub fn image_edit_categories() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGE_EDIT_CATEGORIES))
}

pub fn checkbox(id:&str, label:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(CHECKBOX, &html_map!{
        "label" => label,
        "id" => id
    }).unwrap())
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
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (CATEGORIES, get_template_str(include_str!("../../../.template_output/categories/categories-page.html"))),
            (CATEGORY_MAIN_SELECTED, get_template_str(include_str!("../../../.template_output/categories/category-main-selected.html"))),
            (CATEGORY_MAIN_DESELECTED, get_template_str(include_str!("../../../.template_output/categories/category-main-deselected.html"))),
            (CATEGORY_SUB, get_template_str(include_str!("../../../.template_output/categories/category-sub.html"))),
            (CATEGORY_LABEL_DISPLAY, get_template_str(include_str!("../../../.template_output/categories/category-label-display.html"))),
            (CATEGORY_LABEL_INPUT, get_template_str(include_str!("../../../.template_output/categories/category-label-input.html"))),
            (CATEGORY_MENU, get_template_str(include_str!("../../../.template_output/categories/category-menu.html"))),
            (IMAGES_PAGE, get_template_str(include_str!("../../../.template_output/images/images-page.html"))),
            (IMAGE_ADD, get_template_str(include_str!("../../../.template_output/images/image-add.html"))),
            (IMAGE_EDIT, get_template_str(include_str!("../../../.template_output/images/image-edit.html"))),
            (IMAGE_EDIT_META, get_template_str(include_str!("../../../.template_output/images/image-edit-meta.html"))),
            (IMAGE_EDIT_CATEGORIES, get_template_str(include_str!("../../../.template_output/images/image-edit-categories.html"))),
            (CHECKBOX, get_template_str(include_str!("../../../.template_output/_input/checkbox.html"))),
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
