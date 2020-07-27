use simple_html_template::{TemplateCache, html_map};
use shipyard::EntityId;
use wasm_bindgen::prelude::*;
use web_sys::DocumentFragment;
use crate::dom::selector::entity_id;
use core::settings::SETTINGS;

pub const SIGNIN:&'static str = "signin";
pub const REGISTER:&'static str = "register";

pub struct TemplateManager {
    pub cache: TemplateCache<'static>
}

impl TemplateManager {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![
            (SIGNIN, get_template_str(include_str!("../../../templates/signin_and_registration/signin.html"))),
        ]);

        Self { cache }
    }

    pub fn signin(&self) -> DocumentFragment {
        self.cache.render_dom_plain(SIGNIN)
    }
}

fn get_template_str(s:&'static str) -> &'static str {
    //This leaks memory - which is okay since templates exist for the lifetime of the app
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
