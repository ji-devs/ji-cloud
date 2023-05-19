use crate::stickers::embed::types::{ParseUrlExt, PartialGoogleDocsEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::GoogleDocId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_google_doc_input(
        self: &Rc<Self>,
        google_doc: &Rc<PartialGoogleDocsEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match google_doc.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, google_doc => move |_: events::Input| {
                    match GoogleDocId::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            google_doc.url.set(None);
                        }
                        Ok(google_doc_url) => {
                            actions::set_error(&wrapper, false);
                            google_doc.url.set(Some(google_doc_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }
}
