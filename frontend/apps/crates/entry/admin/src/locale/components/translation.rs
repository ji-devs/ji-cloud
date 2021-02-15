use url::Url;
use crate::locale::state::Section;
use crate::locale::state::{TranslationStatus, ItemKind, Translation, State};
use web_sys::HtmlSelectElement;
use std::rc::Rc;
use std::clone::Clone;
use wasm_bindgen::prelude::*;
use futures_signals::signal::Mutable;
use dominator::{Dom, html, clone, events, with_node};
use std::str::FromStr;
use strum::IntoEnumIterator;


#[derive(Clone)]
pub struct TranslationRow {

}

impl TranslationRow {
    fn url_option_string(url: &Option<Url>) -> String {
        if url.is_some() {
            url.clone().unwrap().to_string()
        } else {
            String::new()
        }
    }

    pub fn render(translation: Rc<Mutable<Translation>>, state: Rc<State>) -> Dom {
        let translation_ref = translation.lock_ref();
        html!("div", {
            .class("ftl-row")
            .children(&mut [
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .property("value", &translation_ref.id)
                        .event(clone!(translation => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.id = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .apply_if(translation_ref.section.is_some(), |dom| {
                            dom.property("value", &translation_ref.section.clone().unwrap())
                        })
                        .attribute("list", "sections")
                        .event(clone!(translation => move |event: events::Input| {
                            let value: Section = event.value().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.section = Some(value);
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .apply_if(translation_ref.item_kind.is_some(), |dom| {
                            dom.property("value", &translation_ref.item_kind.clone().unwrap())
                        })
                        .attribute("list", "translation-kinds")
                        .event(clone!(translation => move |event: events::Input| {
                            let value: ItemKind = event.value().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.item_kind = Some(value);
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("textarea", {
                        .text(&translation_ref.english)
                        .event(clone!(translation => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.english = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("select" => HtmlSelectElement, {
                        .with_node!(elem => {
                            .event(clone!(translation => move |_event: events::Change| {
                                let value: String = elem.value();
                                let mut translation = translation.lock_mut();
                                translation.status = TranslationStatus::from_str(&value).unwrap_throw();
                            }))
                        })
                        .children(
                            TranslationStatus::iter().map(|o| {
                                html!("option", {
                                    .property("text", o.to_string())
                                    .property("value", o.to_string())
                                    .property("selected", o == translation_ref.status)
                                })
                            })
                        )
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .class("zeplin-link-cell")
                    .children(&mut [
                        html!("a", {
                            .attribute("target", "_blank")
                            .class("zeplin-link")
                            .text_signal(translation_ref.zeplin_reference.signal_ref(|url| TranslationRow::url_option_string(url)))
                            .property_signal("href", translation_ref.zeplin_reference.signal_ref(|url| TranslationRow::url_option_string(url)))
                        }),
                        html!("input", {
                            .property("type", "url")
                            .apply_if(translation_ref.zeplin_reference.lock_ref().is_some(), |dom| {
                                dom.property("value", &translation_ref.zeplin_reference.lock_ref().clone().unwrap().to_string())
                            })
                            .event(clone!(translation => move |event: events::Input| {
                                let value: String = event.value().unwrap_throw();
                                let value = Url::parse(&value);

                                let zeplin_reference = &translation.lock_ref().zeplin_reference;
                                match value {
                                    Ok(value) => zeplin_reference.set(Some(value)),
                                    Err(_) => zeplin_reference.set(None),
                                };
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .property("value", &translation_ref.comments)
                        .event(clone!(translation => move |event: events::Input| {
                            let value: String = event.value().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.comments = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", translation_ref.in_app)
                        .event(clone!(translation => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.in_app = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", translation_ref.in_element)
                        .event(clone!(translation => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.in_element = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(html!("input", {
                        .attribute("type", "checkbox")
                        .property("checked", translation_ref.in_mock)
                        .event(clone!(translation => move |event: events::Change| {
                            let value: bool = event.checked().unwrap_throw();
                            let mut translation = translation.lock_mut();
                            translation.in_mock = value;
                        }))
                    }))
                }),
                html!("div", {
                    .class("ftl-cell")
                    .child(
                        html!("div", {
                            .class("actions-wrapper")
                            .children(&mut [
                                html!("button", {
                                    .class("link-button")
                                    .text("Clone")
                                    .event(clone!(state, translation => move |_event: events::Click| {
                                        state.loader.load(clone!(state, translation => async move {
                                            state.clone_translation(&translation.lock_ref()).await;
                                        }))
                                    }))
                                }),
                                html!("span", {
                                    .text("|")
                                }),
                                html!("button", {
                                    .class("link-button")
                                    .text("Delete")
                                    .event(clone!(state, translation => move |_event: events::Click| {
                                        state.remove_translation(&translation.lock_ref().id);
                                    }))
                                }),
                            ])
                        })
                    )
                }),
            ])
        })
    }
}
