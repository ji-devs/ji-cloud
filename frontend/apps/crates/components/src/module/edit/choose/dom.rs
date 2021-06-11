use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;
use shared::domain::jig::module::body::ModeExt;

pub fn render<Mode>(
    state: Rc<Choose<Mode>>
) -> Vec<Dom>
where
    Mode: ModeExt + 'static,
{
    vec![
        html!("choose-mode", {
            .property("slot", "main")
            .property("title", Mode::title())
            .children(
                Mode::get_list()
                    .into_iter()
                    .map(|mode| {
                        html!("choose-mode-option", {
                            .property("mode", mode.as_str_id())
                            .property("label", mode.as_str_label())
                            .property("module", Mode::module_str_id())
                            .event(clone!(state => move |evt:events::Click| {
                                (state.on_mode_change) (mode);
                            }))
                        })
                    })
                    .collect::<Vec<Dom>>()
            )
        })
    ]
}
