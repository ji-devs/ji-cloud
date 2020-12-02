use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;
use crate::data::{GameMode, ContentMode, PairType};

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
macro_rules! template_path {
    ($e:tt) => { 
        concat!("../../../../../../../.template_output/", $e)
    } 
}

const CHOOSE_MODE_PAGE:&'static str = "start-mode-choose";

const HEADER_EMPTY:&'static str = "_common/header/empty";
const HEADER_PREVIEW:&'static str = "_common/header/preview";
const HEADER_ADD_PAIR:&'static str = "_common/header/add-pair";

const MAIN_EMPTY:&'static str = "_common/main/empty";
const MAIN_IFRAME:&'static str = "_common/main/iframe";
const MAIN_PAIRS:&'static str = "_common/main/pairs";
const MAIN_CARD_PAIR_TEXT_TEXT_EDIT:&'static str = "_common/main/card-pairs/text-text-edit";
const MAIN_CARD_PAIR_TEXT_TEXT_PREVIEW:&'static str = "_common/main/card-pairs/text-text-preview";
const MAIN_CARD_PAIR_TEXT_IMAGE_EDIT:&'static str = "_common/main/card-pairs/text-image-edit";
const MAIN_CARD_PAIR_TEXT_IMAGE_PREVIEW:&'static str = "_common/main/card-pairs/text-image-preview";

const FOOTER_DEFAULT:&'static str = "_common/footer/default";

const DUPLICATE_SIDEBAR_STEP_1_EMPTY:&'static str  = "duplicate/sidebar/step1-empty";
const DUPLICATE_SIDEBAR_STEP_1_WORDS:&'static str = "duplicate/sidebar/step1-words";

const WORDS_AND_IMAGES_SIDEBAR_STEP_1_EMPTY:&'static str  = "words-and-images/sidebar/step1-empty";
const WORDS_AND_IMAGES_SIDEBAR_STEP_1_WORDS:&'static str = "words-and-images/sidebar/step1-words";
const WORDS_AND_IMAGES_SIDEBAR_STEP_1_IMAGES:&'static str = "words-and-images/sidebar/step1-images";

const STEP_2_SIDEBAR:&'static str = "_common/sidebar/step2";
const STEP_2_THEME_ITEM_SELECTED:&'static str = "_common/sidebar/step2-theme-item-selected";
const STEP_2_THEME_ITEM_DESELECTED:&'static str = "_common/sidebar/step2-theme-item-deselected";

const STEP_3_SIDEBAR:&'static str = "_common/sidebar/step3";

pub fn choose_mode_page() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(CHOOSE_MODE_PAGE))
}


pub fn header_empty() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(HEADER_EMPTY))
}
pub fn header_preview() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(HEADER_PREVIEW))
}
pub fn header_add_pair() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(HEADER_ADD_PAIR))
}
pub fn footer_default() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(FOOTER_DEFAULT))
}
pub fn main_empty() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_EMPTY))
}
pub fn main_pairs() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_PAIRS))
}
pub fn main_iframe() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_IFRAME))
}

pub fn sidebar_step_1(game_mode:GameMode, content_mode: ContentMode) -> HtmlElement {

    match game_mode {
        GameMode::Duplicate => {
            match content_mode {
                ContentMode::TextInit => {
                    TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_SIDEBAR_STEP_1_WORDS))
                },
                ContentMode::TextDone => {
                    TEMPLATES.with(|t| t.cache.render_elem_plain(DUPLICATE_SIDEBAR_STEP_1_EMPTY))
                },
                _ => unimplemented!("no template!")
            }
        },
        GameMode::WordsAndImages => {
            match content_mode {
                ContentMode::TextInit => {
                    TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_SIDEBAR_STEP_1_WORDS))
                },
                ContentMode::TextDone => {
                    TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_SIDEBAR_STEP_1_EMPTY))
                },
                ContentMode::Images => {
                    TEMPLATES.with(|t| t.cache.render_elem_plain(WORDS_AND_IMAGES_SIDEBAR_STEP_1_IMAGES))
                },
            }
        }
    }
}
pub fn sidebar_step_2() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_2_SIDEBAR))
}
pub fn step_2_theme_item(selected:bool) -> HtmlElement {
    if selected {
        TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_2_THEME_ITEM_SELECTED))
    } else {
        TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_2_THEME_ITEM_DESELECTED))
    }
}
pub fn sidebar_step_3() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(STEP_3_SIDEBAR))
}
pub fn main_pair(pair_type:PairType, is_edit:bool) -> HtmlElement {
    match pair_type {
        PairType::TextText => {
            if is_edit {
                TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_CARD_PAIR_TEXT_TEXT_EDIT))
            } else {
                TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_CARD_PAIR_TEXT_TEXT_PREVIEW))
            }
        },
        PairType::TextImage => {
            if is_edit {
                TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_CARD_PAIR_TEXT_IMAGE_EDIT))
            } else {
                TEMPLATES.with(|t| t.cache.render_elem_plain(MAIN_CARD_PAIR_TEXT_IMAGE_PREVIEW))
            }
        }
    }
}

//replace {{MEDIA_UI}} in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
pub struct Templates {
    pub cache: TemplateCache<'static>
}

impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![

            (CHOOSE_MODE_PAGE, get_template_str(include_str!(template_path!("module/memory/edit/start-mode-choose.html")))),
            (HEADER_EMPTY, get_template_str(include_str!(template_path!("module/memory/edit/_common/header/empty.html")))),
            (HEADER_PREVIEW, get_template_str(include_str!(template_path!("module/memory/edit/_common/header/preview.html")))),
            (HEADER_ADD_PAIR, get_template_str(include_str!(template_path!("module/memory/edit/_common/header/add-pair.html")))),
            (MAIN_EMPTY, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/empty.html")))),
            (MAIN_IFRAME, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/iframe.html")))),
            (MAIN_PAIRS, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/pairs.html")))),
            (MAIN_CARD_PAIR_TEXT_TEXT_EDIT, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/card-pairs/text-text-edit.html")))),
            (MAIN_CARD_PAIR_TEXT_TEXT_PREVIEW, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/card-pairs/text-text-preview.html")))),
            (MAIN_CARD_PAIR_TEXT_IMAGE_EDIT, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/card-pairs/text-image-edit.html")))),
            (MAIN_CARD_PAIR_TEXT_IMAGE_PREVIEW, get_template_str(include_str!(template_path!("module/memory/edit/_common/main/card-pairs/text-image-preview.html")))),

            (FOOTER_DEFAULT, get_template_str(include_str!(template_path!("module/memory/edit/_common/footer/default.html")))),

            (DUPLICATE_SIDEBAR_STEP_1_EMPTY, get_template_str(include_str!(template_path!("module/memory/edit/duplicate/sidebar/step1-empty.html")))),
            (DUPLICATE_SIDEBAR_STEP_1_WORDS, get_template_str(include_str!(template_path!("module/memory/edit/duplicate/sidebar/step1-words.html")))),
            
            (WORDS_AND_IMAGES_SIDEBAR_STEP_1_EMPTY, get_template_str(include_str!(template_path!("module/memory/edit/words-and-images/sidebar/step1-empty.html")))),
            (WORDS_AND_IMAGES_SIDEBAR_STEP_1_WORDS, get_template_str(include_str!(template_path!("module/memory/edit/words-and-images/sidebar/step1-words.html")))),
            (WORDS_AND_IMAGES_SIDEBAR_STEP_1_IMAGES, get_template_str(include_str!(template_path!("module/memory/edit/words-and-images/sidebar/step1-images.html")))),
            (STEP_2_SIDEBAR, get_template_str(include_str!(template_path!("module/memory/edit/_common/sidebar/step2.html")))),
            (STEP_2_THEME_ITEM_SELECTED, get_template_str(include_str!(template_path!("module/memory/edit/_common/sidebar/step2-theme-item-selected.html")))),
            (STEP_2_THEME_ITEM_DESELECTED, get_template_str(include_str!(template_path!("module/memory/edit/_common/sidebar/step2-theme-item-deselected.html")))),

            (STEP_3_SIDEBAR, get_template_str(include_str!(template_path!("module/memory/edit/_common/sidebar/step3.html")))),
        ]);

        Self { cache }
    }
}
/*
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
        TEMPLATES.with(|t| t.cache.render_elem(WORDS_AND_IMAGES_STEP_1_THUMBNAIL, 
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

*/
