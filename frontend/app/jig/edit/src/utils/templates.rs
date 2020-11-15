use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}


const GALLERY_PAGE:&'static str = "gallery";
const EDIT_PAGE:&'static str = "@templates/jig/edit/edit-page.html";
const EDIT_SIDEBAR_SECTION:&'static str = "@templates/jig/edit/sidebar.html";
const EDIT_MENU_SECTION:&'static str = "@templates/jig/edit/menu.html";
const EDIT_DELETE_POPUP:&'static str = "@templates/jig/edit/delete-popup.html";
const EDIT_MODULE_LEFT:&'static str = "@templates/jig/edit/sidebar-module-left.html";
const EDIT_MODULE_RIGHT:&'static str = "@templates/jig/edit/sidebar-module-right.html";
const EDIT_MODULE_DRAG_SLOT:&'static str = "@templates/jig/edit/sidebar-module-drag-slot.html";
const EDIT_MODULE_SELECTION:&'static str = "@templates/jig/edit/module-selection.html";

pub fn gallery() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(GALLERY_PAGE))
}
pub fn edit_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_PAGE))
}
pub fn edit_sidebar_section() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_SIDEBAR_SECTION))
}
pub fn edit_menu_section() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_MENU_SECTION))
}
pub fn edit_delete_popup() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_DELETE_POPUP))
}
pub fn edit_module_left() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_MODULE_LEFT))
}
pub fn edit_module_right() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_MODULE_RIGHT))
}
pub fn edit_module_drag_slot() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_MODULE_DRAG_SLOT))
}
pub fn edit_module_selection() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EDIT_MODULE_SELECTION))
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
            (GALLERY_PAGE, get_template_str(include_str!("../../../../../.template_output/jig/gallery/jig-creator-one.html"))),
            (EDIT_PAGE, get_template_str(include_str!("../../../../../.template_output/jig/edit/edit-page.html"))),
            (EDIT_SIDEBAR_SECTION, get_template_str(include_str!("../../../../../.template_output/jig/edit/sidebar.html"))),
            (EDIT_MENU_SECTION, get_template_str(include_str!("../../../../../.template_output/jig/edit/menu.html"))),
            (EDIT_DELETE_POPUP, get_template_str(include_str!("../../../../../.template_output/jig/edit/delete-popup.html"))),
            (EDIT_MODULE_LEFT, get_template_str(include_str!("../../../../../.template_output/jig/edit/sidebar-module-left.html"))),
            (EDIT_MODULE_RIGHT, get_template_str(include_str!("../../../../../.template_output/jig/edit/sidebar-module-right.html"))),
            (EDIT_MODULE_DRAG_SLOT, get_template_str(include_str!("../../../../../.template_output/jig/edit/sidebar-module-drag-slot.html"))),
            (EDIT_MODULE_SELECTION, get_template_str(include_str!("../../../../../.template_output/jig/edit/module-selection.html"))),
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
