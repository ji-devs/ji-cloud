use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use components::{
    image::search::dom::render as render_image_search,
    text_editor::dom::render_controls as render_text_editor,
    audio_input::dom::render as render_audio_input,
};

use shared::domain::jig::module::body::{ThemeChoice, ThemeId};
pub fn render(state: Rc<Step2>) -> Dom {
    html!("step2-sidebar-container", {
        .children(THEME_IDS.iter().copied()
          .map(|theme_id| {
            html!("step2-sidebar-option", {
                .property("theme", theme_id.as_str_id())
                .property_signal("state", state.base.theme_id_signal().map(clone!(theme_id => move |curr_theme_id| {
                    if curr_theme_id == theme_id {
                        "selected"
                    } else {
                        "idle"
                    }
                })))
                .event(clone!(state => move |evt:events::Click| {
                    state.change_theme(ThemeChoice::Override(theme_id));
                }))
            })
          })
        )
    })
}
