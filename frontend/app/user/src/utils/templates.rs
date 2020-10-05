use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use core::settings::SETTINGS;
use std::fmt;

//pub static TEMPLATES:OnceCell<Templates> = OnceCell::new();

const SIGNIN:&'static str = "signin";

const REGISTER_START:&'static str = "register_start";
const REGISTER_STEP1:&'static str = "register_step1";
const REGISTER_STEP2:&'static str = "register_step2";
const REGISTER_STEP3:&'static str = "register_step3";
const REGISTER_SENT_EMAIL:&'static str = "register_sent_email";
const REGISTER_FINAL:&'static str = "register_final";

const PROFILE:&'static str = "profile";

const EMAIL_CONFIRM:&'static str = "email_confirm";
const EMAIL_CHANGE:&'static str = "email_change";
const FORGOT_PASSWORD:&'static str = "forgot_password";

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}
pub fn signin() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SIGNIN))
}
pub fn register_start() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_START))
}
pub fn register_step1() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_STEP1))
}
pub fn register_step2() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_STEP2))
}
pub fn register_step3() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_STEP3))
}
pub fn register_sent_email() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_SENT_EMAIL))
}
pub fn register_final() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_FINAL))
}
pub fn profile() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PROFILE))
}

pub fn email_confirm() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EMAIL_CONFIRM))
}
pub fn email_change() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(EMAIL_CHANGE))
}

pub fn forgot_password() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(FORGOT_PASSWORD))
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
            (SIGNIN, get_template_str(include_str!("../../../.template_output/signin.html"))),
            (REGISTER_START, get_template_str(include_str!("../../../.template_output/register/register-start.html"))),
            (REGISTER_STEP1, get_template_str(include_str!("../../../.template_output/register/register-1.html"))),
            (REGISTER_STEP2, get_template_str(include_str!("../../../.template_output/register/register-2.html"))),
            (REGISTER_STEP3, get_template_str(include_str!("../../../.template_output/register/register-3.html"))),
            (REGISTER_SENT_EMAIL, get_template_str(include_str!("../../../.template_output/register/register-sent-email.html"))),
            (REGISTER_FINAL, get_template_str(include_str!("../../../.template_output/register/register-final.html"))),
            (PROFILE, get_template_str(include_str!("../../../.template_output/profile.html"))),
            (EMAIL_CONFIRM, get_template_str(include_str!("../../../.template_output/email-confirmation.html"))),
            (EMAIL_CHANGE, get_template_str(include_str!("../../../.template_output/email-change.html"))),
            (FORGOT_PASSWORD, get_template_str(include_str!("../../../.template_output/forgot-password.html"))),
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
