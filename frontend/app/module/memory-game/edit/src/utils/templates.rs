use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}


const MODE_CHOOSE_PAGE:&'static str = "mode-choose-page";

const CARD:&'static str = "card";

const DUPLICATE_STEP_1_PAGE:&'static str = "duplicate-step-1-page";
const DUPLICATE_STEP_1_TOOLTIP:&'static str = "duplicate-step-1-tooltip";
const DUPLICATE_STEP_1_ERROR:&'static str = "duplicate-step-1-error";

pub fn mode_choose_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MODE_CHOOSE_PAGE))
}


pub fn card() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD))
}

pub mod duplicate {
    use super::*;
    pub fn step_1_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_STEP_1_PAGE))
    }
    pub fn step_1_tooltip() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_STEP_1_TOOLTIP))
    }
    pub fn step_1_error() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_STEP_1_ERROR))
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
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (MODE_CHOOSE_PAGE, get_template_str(include_str!("../../../../../../.template_output/module/memory-game/edit/start-mode-choose.html"))),
            (CARD, get_template_str(include_str!("../../../../../../.template_output/module/memory-game/edit/_common/memory-card.html"))),
            (DUPLICATE_STEP_1_PAGE, get_template_str(include_str!("../../../../../../.template_output/module/memory-game/edit/duplicate/step-1.html"))),
            (DUPLICATE_STEP_1_TOOLTIP, get_template_str(include_str!("../../../../../../.template_output/module/memory-game/edit/duplicate/step-1-tooltip.html"))),
            (DUPLICATE_STEP_1_ERROR, get_template_str(include_str!("../../../../../../.template_output/module/memory-game/edit/duplicate/step-1-error.html")))
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
