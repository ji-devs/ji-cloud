use std::{panic, sync::Once};

use web_sys::window;
use console_error_panic_hook;

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
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .set_inner_html("<panic-message></panic-message>");
}
