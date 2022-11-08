use crate::module::_groups::cards::edit::state::{ExtraExt, RawDataExt};

use crate::theme_selector::dom::render_cards as render_theme_selector;
use dominator::{clone, events, html, Dom};
use std::rc::Rc;

use super::state::{Step2, STR_CHANGE_BACKGROUND};

pub fn render<RawData: RawDataExt, E: ExtraExt>(state: Rc<Step2<RawData, E>>) -> Dom {
    html!("module-sidebar-body", {
        .prop("slot", "body")
        .child_signal(state.custom_background.signal_ref(clone!(state => move |custom_background| {
            match custom_background {
                Some(custom_background) => {
                    Some(custom_background.render())
                },
                None => {
                    let action = html!("button-rect", {
                        .prop("kind", "text")
                        .prop("color", "blue")
                        .child(html!("fa-icon", {
                            .prop("icon", "fa-light fa-paint-brush")
                        }))
                        .text(STR_CHANGE_BACKGROUND)
                        .event(clone!(state => move |_: events::Click|{
                            state.open_custom_background();
                        }))
                    });

                    Some(render_theme_selector(state.theme_selector.clone(), None, Some(action)))
                },
            }
        })))
    })
}
