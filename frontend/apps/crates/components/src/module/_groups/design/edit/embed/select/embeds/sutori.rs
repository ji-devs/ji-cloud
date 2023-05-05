use crate::stickers::embed::types::{ParseUrlExt, PartialSutoriEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::SutoriId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_sutori(self: &Rc<Self>, sutori: &Rc<PartialSutoriEmbed>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .prop("slot", "input")
                    .prop("label", "Add a Sutori link")
                    .child(html!("input" => HtmlInputElement, {
                        .prop("value", {
                            // not using a signal because the value can be invalid but should still show up
                            match sutori.url.get_cloned() {
                                Some(url) => url.0.clone(),
                                None => String::new(),
                            }
                        })
                        .with_node!(input => {
                            .event(clone!(state, sutori => move |_: events::Input| {
                                match SutoriId::try_parse(input.value()) {
                                    Err(_) => {
                                        actions::set_error(&wrapper, true);
                                        sutori.url.set(None);
                                    }
                                    Ok(sutori_url) => {
                                        actions::set_error(&wrapper, false);
                                        sutori.url.set(Some(sutori_url));
                                    },
                                };
                                state.on_embed_value_change();
                            }))
                        })
                    }))
                })
            }))
        })
    }
}
