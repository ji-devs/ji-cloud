use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use crate::settings::SETTINGS;
use std::fmt;
use crate::components::module_page::ModulePageKind;

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
