use crate::module::_groups::design::design_ext::DesignExt;
use crate::module::_common::edit::entry::prelude::BaseExt;
use dominator::{clone, html, Dom};
use shared::domain::jig::module::body::StepExt;
use std::rc::Rc;
use utils::prelude::*;

use crate::theme_selector::dom::render_design as render_theme_selector;

use super::ThemeBackground;


const STR_DESIGN_FROM_SCRATCH: &str = "Design from scratch";

impl<Step, Base> ThemeBackground<Step, Base> where
    Step: StepExt + 'static,
    Base: BaseExt<Step> + DesignExt + 'static
{
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("module-sidebar-body", {
            .property("slot", "body")
            .child_signal(state.custom_background.signal_ref(clone!(state => move |custom_background| {
                match custom_background {
                    Some(custom_background) => {
                        Some(custom_background.render())
                    },
                    None => {
                        let action = html!("button-rect", {
                            .property("kind", "text")
                            .property("color", "blue")
                            .child(html!("fa-icon", {
                                .property("icon", "fa-light fa-paint-brush")
                            }))
                            .text(STR_DESIGN_FROM_SCRATCH)
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
}
