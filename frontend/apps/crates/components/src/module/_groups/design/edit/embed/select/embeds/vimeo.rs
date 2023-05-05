use crate::stickers::embed::types::{ParseUrlExt, PartialVimeoEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::VimeoUrl;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_vimeo(self: &Rc<Self>, vimeo: &Rc<PartialVimeoEmbed>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .prop("slot", "input")
                    .prop("label", "Add a Vimeo link")
                    .child(html!("input" => HtmlInputElement, {
                        .prop("value", {
                            // not using a signal because the value can be invalid but should still show up
                            match vimeo.url.get_cloned() {
                                Some(url) => url.0.clone(),
                                None => String::new(),
                            }
                        })
                        .with_node!(input => {
                            .event(clone!(state, vimeo => move |_: events::Input| {
                                log::info!("{} = {:?}", input.value(), VimeoUrl::try_parse(input.value()));
                                match VimeoUrl::try_parse(input.value()) {
                                    Err(_) => {
                                        actions::set_error(&wrapper, true);
                                        vimeo.url.set(None);
                                    }
                                    Ok(vimeo_url) => {
                                        actions::set_error(&wrapper, false);
                                        vimeo.url.set(Some(vimeo_url));
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
