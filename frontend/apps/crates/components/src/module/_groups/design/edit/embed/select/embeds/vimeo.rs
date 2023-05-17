use crate::stickers::embed::types::{ParseUrlExt, PartialVimeoEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::VimeoUrl;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_vimeo_input(
        self: &Rc<Self>,
        vimeo: &Rc<PartialVimeoEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
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
        })
    }
}
