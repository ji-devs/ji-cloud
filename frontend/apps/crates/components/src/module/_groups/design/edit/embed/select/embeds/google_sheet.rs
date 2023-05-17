use crate::stickers::embed::types::{ParseUrlExt, PartialGoogleSheetsEmbed};
use dominator::{clone, html, with_node, Dom};
use shared::domain::module::body::_groups::design::GoogleSheetId;
use std::rc::Rc;
use utils::events;
use web_sys::{HtmlElement, HtmlTextAreaElement};

use super::super::{actions, EmbedSelect};

impl EmbedSelect {
    pub fn render_google_sheet_input(
        self: &Rc<Self>,
        google_sheet: &Rc<PartialGoogleSheetsEmbed>,
        wrapper: HtmlElement,
    ) -> Dom {
        let state = self;
        html!("textarea" => HtmlTextAreaElement, {
            .prop("placeholder", "Place the link here")
            .prop("value", {
                // not using a signal because the value can be invalid but should still show up
                match google_sheet.url.get_cloned() {
                    Some(url) => url.0.clone(),
                    None => String::new(),
                }
            })
            .with_node!(input => {
                .event(clone!(state, google_sheet => move |_: events::Input| {
                    match GoogleSheetId::try_parse(input.value()) {
                        Err(_) => {
                            actions::set_error(&wrapper, true);
                            google_sheet.url.set(None);
                        }
                        Ok(google_sheet_url) => {
                            actions::set_error(&wrapper, false);
                            google_sheet.url.set(Some(google_sheet_url));
                        },
                    };
                    state.on_embed_value_change();
                }))
            })
        })
    }
}
