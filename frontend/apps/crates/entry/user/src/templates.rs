use simple_html_template::{TemplateCache, html_map};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;
use utils::settings::SETTINGS;
use std::fmt;

//pub static TEMPLATES:OnceCell<Templates> = OnceCell::new();

const CHECKBOX:&'static str = "checkbox";
const SIGNIN:&'static str = "signin";

const REGISTER_START:&'static str = "register_start";
const REGISTER_STEP1:&'static str = "register_step1";
const REGISTER_STEP2:&'static str = "register_step2";
const REGISTER_STEP3:&'static str = "register_step3";
const REGISTER_FINAL:&'static str = "register_final";

const PROFILE:&'static str = "profile";
const PROFILE_EMAIL_CHANGE:&'static str = "profile_email_change";

const SEND_EMAIL_CONFIRMATION:&'static str = "send_email_confirmation";
const GOT_EMAIL_CONFIRMATION:&'static str = "got_email_confirmation";
const FORGOT_PASSWORD:&'static str = "forgot_password";

thread_local! {
    pub static TEMPLATES: Templates = Templates::new(); 
}

pub fn checkbox(id:&str, label:&str) -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem(CHECKBOX, &html_map!{
        "label" => label,
        "id" => id
    }).unwrap())
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
pub fn register_final() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(REGISTER_FINAL))
}
pub fn profile() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PROFILE))
}

pub fn send_email_confirmation() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(SEND_EMAIL_CONFIRMATION))
}
pub fn got_email_confirmation() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(GOT_EMAIL_CONFIRMATION))
}
pub fn profile_email_change() -> HtmlElement {
    TEMPLATES.with(|t| t.cache.render_elem_plain(PROFILE_EMAIL_CHANGE))
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
            (SIGNIN, get_template_str(include_str!("../../../../../.template_output/user/signin/signin.html"))),
            (REGISTER_START, get_template_str(include_str!("../../../../../.template_output/user/register/register-start.html"))),
            (REGISTER_STEP1, get_template_str(include_str!("../../../../../.template_output/user/register/register-1.html"))),
            (REGISTER_STEP2, get_template_str(include_str!("../../../../../.template_output/user/register/register-2.html"))),
            (REGISTER_STEP3, get_template_str(include_str!("../../../../../.template_output/user/register/register-3.html"))),
            (REGISTER_FINAL, get_template_str(include_str!("../../../../../.template_output/user/register/register-final.html"))),
            (PROFILE, get_template_str(include_str!("../../../../../.template_output/user/profile/profile.html"))),
            (PROFILE_EMAIL_CHANGE, get_template_str(include_str!("../../../../../.template_output/user/profile/email-change.html"))),
            (SEND_EMAIL_CONFIRMATION, get_template_str(include_str!("../../../../../.template_output/user/misc/send-email-confirmation.html"))),
            (GOT_EMAIL_CONFIRMATION, get_template_str(include_str!("../../../../../.template_output/user/misc/got-email-confirmation.html"))),
            (FORGOT_PASSWORD, get_template_str(include_str!("../../../../../.template_output/user/misc/forgot-password.html"))),

            (CHECKBOX, get_template_str(include_str!("../../../../../.template_output/_common/input/checkbox.html"))),
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
