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
        concat!("../../../../../.template_output/", $e)
    } 
}

const CONTAINER:&'static str = "container";
const SIDEBAR_LINK:&'static str = "sidebar-link";
const SIDEBAR_LINK_LOCKED:&'static str = "sidebar-link-locked";

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
const IMAGE_EDIT_CATEGORIES_CHILD:&'static str = "image-edit-categories-child";
const IMAGE_EDIT_CATEGORIES_CHILD_END:&'static str = "image-edit-categories-child-end";
const IMAGE_EDIT_CATEGORIES_PARENT:&'static str = "image-edit-categories-parent";
const IMAGE_EDIT_CATEGORIES_PARENT_END:&'static str = "image-edit-categories-parent-end";
const IMAGE_EDIT_CATEGORIES_SUMMARY_CHILD:&'static str = "image-edit-categories-sum-child";
const IMAGE_EDIT_CATEGORIES_SUMMARY_PARENT:&'static str = "image-edit-categories-sum-parent";
const IMAGE_EDIT_OVERVIEW:&'static str = "image-edit-overview";
const IMAGES_SEARCH:&'static str = "images-search";
const IMAGE_GRID_ITEM_RED:&'static str = "image-grid-item-red";
const IMAGE_GRID_ITEM_GREEN:&'static str = "image-grid-item-green";


pub fn container() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CONTAINER))
}
pub fn sidebar_link(label:&str, href:&str, locked: bool) -> HtmlElement {
    TEMPLATES.with(|t| {
        if locked {
            t.cache.render_elem(SIDEBAR_LINK_LOCKED, &html_map!(
                "label" => label,
            ))
        } else {
            t.cache.render_elem(SIDEBAR_LINK, &html_map!(
                "label" => label,
                "url" => href,
            ))
        }
    }.unwrap())
}

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

pub fn image_edit_category_child(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_CHILD, &html_map!{
        "name" => name
    }).unwrap_throw())
}
pub fn image_edit_category_child_end(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_CHILD_END, &html_map!{
        "name" => name
    }).unwrap_throw())
}

pub fn image_edit_category_parent(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_PARENT, &html_map!{
        "name" => name
    }).unwrap_throw())
}
pub fn image_edit_category_parent_end(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_PARENT_END, &html_map!{
        "name" => name
    }).unwrap_throw())
}
pub fn image_edit_category_summary_parent(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_SUMMARY_PARENT, &html_map!{
        "name" => name
    }).unwrap_throw())
}
pub fn image_edit_category_summary_child(name:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_CATEGORIES_SUMMARY_CHILD, &html_map!{
        "name" => name
    }).unwrap_throw())
}
pub fn image_edit_overview(name:&str, description:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_EDIT_OVERVIEW, &html_map!{
        "name" => name,
        "description" => description,
    }).unwrap_throw())
}

pub fn images_search() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(IMAGES_SEARCH))
}
pub fn image_grid_item_green(src:&str, label:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_GRID_ITEM_GREEN, &html_map!{
        "src" => src,
        "label" => label,
    }).unwrap_throw())
}
pub fn image_grid_item_red(src:&str, label:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(IMAGE_GRID_ITEM_RED, &html_map!{
        "src" => src,
        "label" => label,
    }).unwrap_throw())
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
            (CONTAINER, include_str!(template_path!("admin/container.html"))),
            (SIDEBAR_LINK, include_str!(template_path!("admin/sidebar-link.html"))),
            (SIDEBAR_LINK_LOCKED, include_str!(template_path!("admin/sidebar-link-locked.html"))),
            (CATEGORIES, include_str!(template_path!("admin/categories/categories-page.html"))),
            (CATEGORY_MAIN_SELECTED, include_str!(template_path!("admin/categories/category-main-selected.html"))),
            (CATEGORY_MAIN_DESELECTED, include_str!(template_path!("admin/categories/category-main-deselected.html"))),
            (CATEGORY_SUB, include_str!(template_path!("admin/categories/category-sub.html"))),
            (CATEGORY_LABEL_DISPLAY, include_str!(template_path!("admin/categories/category-label-display.html"))),
            (CATEGORY_LABEL_INPUT, include_str!(template_path!("admin/categories/category-label-input.html"))),
            (CATEGORY_MENU, include_str!(template_path!("admin/categories/category-menu.html"))),
            (IMAGES_PAGE, include_str!(template_path!("admin/images/images-page.html"))),
            (IMAGE_ADD, include_str!(template_path!("admin/images/image-add.html"))),
            (IMAGE_EDIT, include_str!(template_path!("admin/images/image-edit.html"))),
            (IMAGE_EDIT_META, include_str!(template_path!("admin/images/image-edit-meta.html"))),
            (IMAGE_EDIT_CATEGORIES, include_str!(template_path!("admin/images/image-edit-categories.html"))),
            (IMAGE_EDIT_CATEGORIES_CHILD, include_str!(template_path!("admin/images/image-edit-categories-child.html"))),
            (IMAGE_EDIT_CATEGORIES_CHILD_END, include_str!(template_path!("admin/images/image-edit-categories-child-end.html"))),
            (IMAGE_EDIT_CATEGORIES_PARENT, include_str!(template_path!("admin/images/image-edit-categories-parent.html"))),
            (IMAGE_EDIT_CATEGORIES_PARENT_END, include_str!(template_path!("admin/images/image-edit-categories-parent-end.html"))),
            (IMAGE_EDIT_CATEGORIES_SUMMARY_CHILD, include_str!(template_path!("admin/images/image-edit-categories-sum-child.html"))),
            (IMAGE_EDIT_CATEGORIES_SUMMARY_PARENT, include_str!(template_path!("admin/images/image-edit-categories-sum-parent.html"))),
            (IMAGE_EDIT_OVERVIEW, include_str!(template_path!("admin/images/image-edit-overview.html"))),
            (IMAGES_SEARCH, include_str!(template_path!("admin/images/images-search.html"))),
            (IMAGE_GRID_ITEM_RED, include_str!(template_path!("_common/image/image-grid-item-red.html"))),
            (IMAGE_GRID_ITEM_GREEN, include_str!(template_path!("_common/image/image-grid-item-green.html"))),
            (CHECKBOX, include_str!(template_path!("_common/input/checkbox.html"))),
        ]);

        Self { cache }
    }

}