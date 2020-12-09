use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;
use components::module::page::ModulePageKind;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../html/", $e)
    } 
}

const EDIT_PLAIN_HEADER:&'static str = "edit-plain-header";
const EDIT_RESIZE_HEADER:&'static str = "edit-resize-header";
const EDIT_PLAIN_FOOTER:&'static str = "edit-plain-footer";
const EDIT_RESIZE_FOOTER:&'static str = "edit-resize-footer";
const EDIT_PLAIN_SIDEBAR:&'static str = "edit-plain-sidebar";
const EDIT_RESIZE_SIDEBAR:&'static str = "edit-resize-sidebar";
const EDIT_PLAIN_MAIN:&'static str = "edit-plain-main";
const EDIT_RESIZE_MAIN:&'static str = "edit-resize-main";
const IFRAME_MAIN:&'static str = "iframe-main";

pub fn header(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::EditPlain => Some(EDIT_PLAIN_HEADER),
        ModulePageKind::EditResize => Some(EDIT_RESIZE_HEADER),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}
pub fn footer(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::EditPlain => Some(EDIT_PLAIN_FOOTER),
        ModulePageKind::EditResize => Some(EDIT_RESIZE_FOOTER),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}
pub fn sidebar(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::EditPlain => Some(EDIT_PLAIN_SIDEBAR),
        ModulePageKind::EditResize => Some(EDIT_RESIZE_SIDEBAR),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}

pub fn main(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::EditPlain => Some(EDIT_PLAIN_MAIN),
        ModulePageKind::EditResize => Some(EDIT_RESIZE_MAIN),
        ModulePageKind::PlayIframe => Some(IFRAME_MAIN),
        ModulePageKind::PlayIframePreview => Some(IFRAME_MAIN),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
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
            (EDIT_PLAIN_HEADER, include_str!(template_path!("edit-plain/header.html"))),
            (EDIT_RESIZE_HEADER, include_str!(template_path!("edit-resize/header.html"))),
            (EDIT_PLAIN_FOOTER, include_str!(template_path!("edit-plain/footer.html"))),
            (EDIT_RESIZE_FOOTER, include_str!(template_path!("edit-resize/footer.html"))),
            (EDIT_PLAIN_SIDEBAR, include_str!(template_path!("edit-plain/sidebar.html"))),
            (EDIT_RESIZE_SIDEBAR, include_str!(template_path!("edit-resize/sidebar.html"))),
            (EDIT_PLAIN_MAIN, include_str!(template_path!("edit-plain/main.html"))),
            (EDIT_RESIZE_MAIN, include_str!(template_path!("edit-resize/main.html"))),
            (IFRAME_MAIN, include_str!(template_path!("iframe/main.html"))),
        ]);

        Self { cache }
    }

}
