use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

const MODULE_PAGE:&'static str = "module-page";

pub fn module_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MODULE_PAGE))
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
            (MODULE_PAGE, get_template_str(include_str!("../../../../../.template_output/_common/module/module-page.html"))),
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
