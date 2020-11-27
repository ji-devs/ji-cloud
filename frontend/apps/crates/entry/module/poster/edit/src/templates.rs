use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../../../.template_output/", $e)
    } 
}


const SIDEBAR:&'static str = "sidebar";
const HEADER:&'static str = "header";
const FOOTER:&'static str = "footer";
const MAIN:&'static str = "main";
const SIDEBAR_LAYOUT:&'static str = "sidebar-layout";
const SIDEBAR_LAYOUT_ITEM:&'static str = "sidebar-layout-item";

pub fn sidebar() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR))
}

pub fn header(title:&str, subtitle:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(HEADER, &html_map!(
        "title" => title,
        "subtitle" => subtitle
    )).unwrap_throw())
}
pub fn footer() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(FOOTER))
}
pub fn main() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN))
}
pub fn sidebar_layout() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR_LAYOUT))
}
pub fn sidebar_layout_item() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIDEBAR_LAYOUT_ITEM))
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
            (SIDEBAR, get_template_str(include_str!(
                template_path!("module/poster/edit/sidebar/sidebar.html")
            ))),
            (HEADER, get_template_str(include_str!(
                template_path!("module/poster/edit/header.html")
            ))),
            (FOOTER, get_template_str(include_str!(
                template_path!("module/poster/edit/footer.html")
            ))),
            (MAIN, get_template_str(include_str!(
                template_path!("module/poster/edit/main.html")
            ))),
            (SIDEBAR_LAYOUT, get_template_str(include_str!(
                template_path!("module/poster/edit/sidebar/layout.html")
            ))),
            (SIDEBAR_LAYOUT_ITEM, get_template_str(include_str!(
                template_path!("module/poster/edit/sidebar/layout-item.html")
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
