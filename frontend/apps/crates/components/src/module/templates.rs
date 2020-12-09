use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;
use super::page::ModulePageKind;

thread_local! {
    static TEMPLATES: Templates = Templates::new(); 
}
pub struct Templates {
    pub cache: TemplateCache<'static>
}

macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../.template_output/", $e)
    } 
}
/*
import moduleEditPagePlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";
import modulePlayPage from "@templates/module/_common/module-play-page.html";
*/
const PAGE_EMPTY:&'static str = "module-page-empty";
const EDIT_PAGE_PLAIN:&'static str = "module-edit-page-plain";
const EDIT_PAGE_RESIZE:&'static str = "module-edit-page-resize";
const PLAY_IFRAME:&'static str = "module-play-iframe";
const PLAY_IFRAME_PREVIEW:&'static str = "module-play-iframe-preview";

pub fn page(kind:ModulePageKind) -> HtmlElement {
    match kind {
        ModulePageKind::Empty => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(PAGE_EMPTY))
        },
        ModulePageKind::EditPlain => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_PAGE_PLAIN))
        },
        ModulePageKind::EditResize => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_PAGE_RESIZE))
        },
        ModulePageKind::PlayIframe => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(PLAY_IFRAME))
        },
        ModulePageKind::PlayIframePreview => {
            TEMPLATES.with(|t| t.cache.render_elem_plain(PLAY_IFRAME_PREVIEW))
        }
    }
}

impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (PAGE_EMPTY, include_str!(
                template_path!("module/_common/module-page-empty.html")
            )),
            (EDIT_PAGE_PLAIN, include_str!(
                template_path!("module/_common/module-edit-page-plain.html")
            )),
            (EDIT_PAGE_RESIZE, include_str!(
                template_path!("module/_common/module-edit-page-resize.html")
            )),
            (PLAY_IFRAME, include_str!(
                template_path!("module/_common/module-play-iframe.html")
            )),
            (PLAY_IFRAME_PREVIEW, include_str!(
                template_path!("module/_common/module-play-iframe-preview.html")
            )),
        ]);

        Self { cache }
    }

}