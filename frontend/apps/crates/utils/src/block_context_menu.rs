use crate::js_wrappers::set_event_listener;
use gloo::utils::window;
use web_sys::Event;

pub fn block_context_menu_globally() {
    set_event_listener(
        &window(),
        "contextmenu",
        Box::new(|e: Event| {
            e.prevent_default();
        }),
    )
}
