use crate::stickers::embed::types::{ParseUrlExt, PartialGoogleSlidesEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::GoogleSlideId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_google_slide_input(
        self: &Rc<Self>,
        google_slide: &Rc<PartialGoogleSlidesEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match google_slide.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, google_slide => move |_: events::Input| {
                    match GoogleSlideId::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            google_slide.url.set(None);
                        }
                        Ok(google_slide_url) => {
                            actions::set_error(&wrapper, false);
                            google_slide.url.set(Some(google_slide_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }
}
