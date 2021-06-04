use super::state::*;
use std::rc::Rc;
use dominator::{html, clone, Dom};
use utils::prelude::*;

pub fn render<Mode, RawMode>(
    state: Rc<Choose<Mode, RawMode>>
) -> Vec<Dom>
where
    Mode: ModeExt<RawMode> + 'static,
    RawMode: 'static
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
                            .property("module", Mode::module())
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
