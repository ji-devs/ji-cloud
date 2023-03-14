use std::{rc::Rc, str::FromStr};

use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{not, Signal},
};
use url::{ParseError, Url};
use utils::events;
use web_sys::HtmlTextAreaElement;

use super::state::AddLink;

const STR_ADD_LINK: &str = "Insert URL here";

impl AddLink {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = Rc::clone(self);
        html!("div", {
            .children(&mut [
                html!("textarea" => HtmlTextAreaElement, {
                    .with_node!(elem => {
                        .prop("slot", "textarea")
                        .prop("spellcheck", "false")
                        .style("width", "95%")
                        .style("background-color", "var(--light-blue-2)")
                        .style("border-radius", "8px")
                        .style("padding", "10px 15px")
                        .style("font-size", "16px")
                        .style("font-weight", "500")
                        .style("color", "var(--dark-gray-6)")
                        .style("border", "0")
                        .style("height", "100px")
                        .style("resize", "none")
                        .event(clone!(state, elem => move |_: events::Change| {
                            let val = elem.value().trim().to_string();
                            let url = Url::from_str(&val);

                            match url {
                                Ok(url) => {
                                    let _ = elem.remove_attribute("error");
                                    state.url.set(Some(url));
                                },
                                Err(err) => {
                                    match err {
                                        ParseError::RelativeUrlWithoutBase => {
                                            let url_with_https = prepend_https_to_url(&val);
                                            let _ = elem.remove_attribute("error");
                                            elem.set_value(url_with_https.as_str());
                                            state.url.set(Some(url_with_https));
                                        },
                                        _ => {
                                            let _ = elem.set_attribute("error", "");
                                            state.url.set(None);
                                        },
                                    }
                                },
                            }
                        }))
                    })
                }),
            ])
        })
    }
}

fn prepend_https_to_url(url: &str) -> Url {
    let mut fixed_url_string = String::new();
    fixed_url_string.push_str("https://");
    fixed_url_string.push_str(url);
    Url::from_str(&fixed_url_string).unwrap()
}
