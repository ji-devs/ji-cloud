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

const MODE_CHOOSE_PAGE:&'static str = "mode-choose-page";


// used in all modes
const CARD_PAIR_TEXT_TEXT_EDIT:&'static str = "card-pair-text-text-edit";
const CARD_PAIR_TEXT_TEXT_PREVIEW:&'static str = "card-pair-text-text-preview";
const CARD_PAIR_TEXT_IMAGE_EDIT:&'static str = "card-pair-text-image-edit";
const CARD_PAIR_TEXT_IMAGE_PREVIEW:&'static str = "card-pair-text-image-preview";


const STEP_2_THEME_ITEM_SELECTED:&'static str = "step-2-theme-item-selected";
const STEP_2_THEME_ITEM_DESELECTED:&'static str = "step-2-theme-item-deselected";

// duplicate mode
const DUPLICATE_STEP_1_PAGE:&'static str = "duplicate-step-1-page";
const DUPLICATE_STEP_1_TOOLTIP:&'static str = "duplicate-step-1-tooltip";
const DUPLICATE_STEP_1_ERROR:&'static str = "duplicate-step-1-error";
const DUPLICATE_STEP_2_PAGE:&'static str = "duplicate-step-2-page";
const DUPLICATE_STEP_4_PAGE:&'static str = "duplicate-step-4-page";

// words and images mode
const WORDS_AND_IMAGES_STEP_1_PAGE:&'static str = "words-and-images-step-1-page";
const WORDS_AND_IMAGES_STEP_2_PAGE:&'static str = "words-and-images-step-2-page";
const WORDS_AND_IMAGES_STEP_4_PAGE:&'static str = "words-and-images-step-4-page";
const WORDS_AND_IMAGES_STEP_1_THUMBNAIL:&'static str = "words-and-images-step-1-thumbnail";

pub fn mode_choose_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MODE_CHOOSE_PAGE))
}


pub fn card_pair_text_text_edit() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD_PAIR_TEXT_TEXT_EDIT))
}
pub fn card_pair_text_text_preview() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD_PAIR_TEXT_TEXT_PREVIEW))
}
pub fn card_pair_text_image_edit() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD_PAIR_TEXT_IMAGE_EDIT))
}

pub fn card_pair_text_image_preview() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CARD_PAIR_TEXT_IMAGE_PREVIEW))
}

pub fn step_2_theme_item(selected:bool) -> HtmlElement {
    if selected {
        TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_2_THEME_ITEM_SELECTED))
    } else {
        TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_2_THEME_ITEM_DESELECTED))
    }
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
    pub fn step_2_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_STEP_2_PAGE))
    }
    pub fn step_4_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_STEP_4_PAGE))
    }


}

pub mod words_and_images {
    use super::*;
    pub fn step_1_thumbnail(src:&str) -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem(WORDS_AND_IMAGES_STEP_1_PAGE, 
            &html_map!(
                "src" => src
            )
        ).unwrap_throw())
    }
    pub fn step_1_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_STEP_1_PAGE))
    }
    pub fn step_2_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_STEP_2_PAGE))
    }
    pub fn step_4_page() -> HtmlElement {
        TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_STEP_4_PAGE))
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
            (MODE_CHOOSE_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/start-mode-choose.html")
            ))),


            (CARD_PAIR_TEXT_TEXT_EDIT, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/card-pairs/text-text-edit.html")
            ))),
            (CARD_PAIR_TEXT_TEXT_PREVIEW, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/card-pairs/text-text-preview.html")
            ))),
            (CARD_PAIR_TEXT_IMAGE_EDIT, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/card-pairs/text-image-edit.html")
            ))),
            (CARD_PAIR_TEXT_IMAGE_PREVIEW, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/card-pairs/text-image-preview.html")
            ))),

            (STEP_2_THEME_ITEM_SELECTED, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/sidebar/step-2-theme-item-selected.html")
            ))),
            (STEP_2_THEME_ITEM_DESELECTED, get_template_str(include_str!(
                template_path!("module/memory/edit/_common/sidebar/step-2-theme-item-deselected.html")
            ))),

            (DUPLICATE_STEP_1_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/duplicate/step-1.html")
            ))),
            (DUPLICATE_STEP_1_TOOLTIP, get_template_str(include_str!(
                template_path!("module/memory/edit/duplicate/step-1-tooltip.html")
            ))),
            (DUPLICATE_STEP_1_ERROR, get_template_str(include_str!(
                template_path!("module/memory/edit/duplicate/step-1-error.html")
            ))),
            (DUPLICATE_STEP_2_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/duplicate/step-2.html")
            ))),
            (DUPLICATE_STEP_4_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/duplicate/step-4.html")
            ))),

            (WORDS_AND_IMAGES_STEP_1_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/words-and-images/step-1/step-1.html")
            ))),
            (WORDS_AND_IMAGES_STEP_1_THUMBNAIL, get_template_str(include_str!(
                template_path!("module/memory/edit/words-and-images/step-1/sidebar/image-thumbnail.html")
            ))),
            (WORDS_AND_IMAGES_STEP_2_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/words-and-images/step-2.html")
            ))),
            (WORDS_AND_IMAGES_STEP_4_PAGE, get_template_str(include_str!(
                template_path!("module/memory/edit/words-and-images/step-4.html")
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
