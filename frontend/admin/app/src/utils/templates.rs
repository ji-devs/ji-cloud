use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
pub fn categories() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CATEGORIES))
}
pub fn category(id:&str, name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(CATEGORY, &html_map!(
        "id" => id,
        "name" => name,
    )).unwrap())
}

//pub static TEMPLATES:OnceCell<Templates> = OnceCell::new();

const CATEGORIES:&'static str = "categories";
const CATEGORY:&'static str = "category";

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
            (CATEGORY, get_template_str(include_str!("../../../.template_output/categories/category.html"))),
        ]);

        Self { cache }
    }

}

//replace %MEDIA_UI% in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
