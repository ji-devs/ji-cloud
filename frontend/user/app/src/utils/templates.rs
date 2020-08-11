use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use core::settings::SETTINGS;
use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
pub fn signin() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIGNIN))
}
pub fn register() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER))
}
pub fn profile() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PROFILE))
}

//pub static TEMPLATES:OnceCell<Templates> = OnceCell::new();

const SIGNIN:&'static str = "signin";
const WAIT:&'static str = "wait";
const REGISTER:&'static str = "register";
const PROFILE:&'static str = "profile";

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
            (SIGNIN, get_template_str(include_str!("../../../.template_output/signin_and_registration/signin.html"))),
            (REGISTER, get_template_str(include_str!("../../../.template_output/signin_and_registration/register.html"))),
            (WAIT, get_template_str(include_str!("../../../.template_output/signin_and_registration/wait.html"))),
            (PROFILE, get_template_str(include_str!("../../../.template_output/profile.html"))),
        ]);

        Self { cache }
    }

}

//replace %MEDIA_UI% in the template string
//this leaks memory - which is okay since templates exist for the lifetime of the app
fn get_template_str(s:&'static str) -> &'static str {
    unsafe {
        Box::leak(SETTINGS.get_unchecked().remote_target.replace_media_ui(s).into_boxed_str())
    }
}
