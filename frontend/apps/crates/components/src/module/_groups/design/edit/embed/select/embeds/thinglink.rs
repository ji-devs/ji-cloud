use crate::stickers::embed::types::{ParseUrlExt, PartialThinglinkEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::ThinglinkId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_thinglink_input(
        self: &Rc<Self>,
        thinglink: &Rc<PartialThinglinkEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match thinglink.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, thinglink => move |_: events::Input| {
                    match ThinglinkId::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            thinglink.url.set(None);
                        }
                        Ok(thinglink_url) => {
                            actions::set_error(&wrapper, false);
                            thinglink.url.set(Some(thinglink_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }
}
