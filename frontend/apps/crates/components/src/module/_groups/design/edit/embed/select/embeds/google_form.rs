use crate::stickers::embed::types::{ParseUrlExt, PartialGoogleFormsEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::GoogleFormId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_google_form_input(
        self: &Rc<Self>,
        google_form: &Rc<PartialGoogleFormsEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match google_form.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, google_form => move |_: events::Input| {
                    match GoogleFormId::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            google_form.url.set(None);
                        }
                        Ok(google_form_url) => {
                            actions::set_error(&wrapper, false);
                            google_form.url.set(Some(google_form_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }
}
