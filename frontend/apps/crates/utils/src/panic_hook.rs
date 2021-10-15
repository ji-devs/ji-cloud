use std::{panic, sync::Once};

use console_error_panic_hook;
use web_sys::{window, HtmlElement};

pub fn set_hook() {
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        panic::set_hook(Box::new(hook));
    });
}

fn hook(info: &panic::PanicInfo) {
    show_panic_message();
    console_error_panic_hook::hook(info);
}

fn show_panic_message() {
    if let Some(body) = get_body() {
        body.set_inner_html("<panic-message></panic-message>");
    }
}

fn get_body() -> Option<HtmlElement> {
    window()?.document()?.body()
}
