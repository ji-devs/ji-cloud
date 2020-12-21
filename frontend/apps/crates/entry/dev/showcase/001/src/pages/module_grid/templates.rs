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
        concat!("../../../html/module-grid/", $e)
    } 
}

const GRID_PLAIN_HEADER:&'static str = "grid-plain-header";
const GRID_RESIZE_HEADER:&'static str = "grid-resize-header";
const GRID_PLAIN_FOOTER:&'static str = "grid-plain-footer";
const GRID_RESIZE_FOOTER:&'static str = "grid-resize-footer";
const GRID_PLAIN_SIDEBAR:&'static str = "grid-plain-sidebar";
const GRID_RESIZE_SIDEBAR:&'static str = "grid-resize-sidebar";
const GRID_PLAIN_MAIN:&'static str = "grid-plain-main";
const GRID_RESIZE_MAIN:&'static str = "grid-resize-main";
const IFRAME_MAIN:&'static str = "iframe-main";

pub fn header(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::GridPlain | ModulePageKind::Empty => Some(GRID_PLAIN_HEADER),
        ModulePageKind::GridResize | ModulePageKind::GridResizeScrollable => Some(GRID_RESIZE_HEADER),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}
pub fn footer(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::GridPlain  | ModulePageKind::Empty => Some(GRID_PLAIN_FOOTER),
        ModulePageKind::GridResize | ModulePageKind::GridResizeScrollable => Some(GRID_RESIZE_FOOTER),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}
pub fn sidebar(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::GridPlain  | ModulePageKind::Empty => Some(GRID_PLAIN_SIDEBAR),
        ModulePageKind::GridResize | ModulePageKind::GridResizeScrollable => Some(GRID_RESIZE_SIDEBAR),
        _ => None 
    };

    id.map(|id| {
        TEMPLATES.with(|t| t.cache.render_elem_plain(id))
    })
}

pub fn main(kind:ModulePageKind) -> Option<HtmlElement> {
    let id = match kind {
        ModulePageKind::GridPlain  | ModulePageKind::Empty => Some(GRID_PLAIN_MAIN),
        ModulePageKind::GridResize | ModulePageKind::GridResizeScrollable => Some(GRID_RESIZE_MAIN),
        ModulePageKind::Iframe => Some(IFRAME_MAIN),
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
            (GRID_PLAIN_HEADER, include_str!(template_path!("grid-plain/header.html"))),
            (GRID_RESIZE_HEADER, include_str!(template_path!("grid-resize/header.html"))),
            (GRID_PLAIN_FOOTER, include_str!(template_path!("grid-plain/footer.html"))),
            (GRID_RESIZE_FOOTER, include_str!(template_path!("grid-resize/footer.html"))),
            (GRID_PLAIN_SIDEBAR, include_str!(template_path!("grid-plain/sidebar.html"))),
            (GRID_RESIZE_SIDEBAR, include_str!(template_path!("grid-resize/sidebar.html"))),
            (GRID_PLAIN_MAIN, include_str!(template_path!("grid-plain/main.html"))),
            (GRID_RESIZE_MAIN, include_str!(template_path!("grid-resize/main.html"))),
            (IFRAME_MAIN, include_str!(template_path!("iframe/main.html"))),
        ]);

        Self { cache }
    }

}
