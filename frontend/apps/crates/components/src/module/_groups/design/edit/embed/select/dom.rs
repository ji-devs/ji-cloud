use std::rc::Rc;

use crate::stickers::embed::types::PartialEmbedHost;
use const_format::formatcp;
use dominator::{clone, html, with_node, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use itertools::Itertools;
use utils::{component::Component, events};
use web_sys::{HtmlElement, ShadowRoot};

use super::{EmbedHostType, EmbedSelect};

macro_rules! gap {
    ($size:expr) => {
        {
            const SIZE_U32: u32 = $size; // needed to asset type
            const SIZE_STR: &str = formatcp!("{}px", SIZE_U32);
            html!("div", {
                .style("height", SIZE_STR)
                .style("width", SIZE_STR)
            })
        }
    };
}

const STR_SELECT_TITLE: &str = "Click to embed:";
const STR_DELETE: &str = "Delete";
const STR_BACK: &str = "Back";

impl Component<EmbedSelect> for Rc<EmbedSelect> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;
        dom.child(html!("div", {
            .class("main")
            .child_signal(state.host.signal_cloned().map(clone!(state => move |embed| {
                Some(match embed {
                    Some(embed) => {
                        state.render_host(embed)
                    },
                    None => {
                        state.render_options()
                    },
                })
            })))
        }))
    }
}

impl EmbedSelect {
    fn render_options(self: &Rc<Self>) -> Dom {
        let state: &Rc<EmbedSelect> = self;
        html!("div", {
            .class("select-wrapper")
            .child(gap!(12))
            .child(html!("h4", {
                .text(STR_SELECT_TITLE)
            }))
            .child(gap!(44))
            .child(html!("div", {
                .class("options-wrapper")
                .children(state
                    .type_list
                    .modules
                    .iter()
                    .map(clone!(state => move|module| {
                        state.option(module)
                    }))
                    .collect_vec()
                )
            }))
        })
    }

    fn option(self: &Rc<Self>, embed_type: &EmbedHostType) -> Dom {
        let state = self;
        html!("label", {
            .children(&mut [
                html!("img-ui", {
                    .prop("path", &format!(
                        "module/_common/edit/widgets/sidebar/embed/square/{}.png",
                        embed_type.as_str()
                    ))
                    .style_signal("outline", state.host.signal_ref(clone!(embed_type => move |host| {
                        let is_selected = match host {
                            Some(host) => embed_type == partial_to_type(host),
                            None => false,
                        };
                        match is_selected {
                            true => "solid 3px var(--light-blue-5)",
                            false => "none",
                        }
                    })))
                }),
                html!("p", {
                    .text(embed_type.display_name())
                }),
                html!("input", {
                    .prop("type", "radio")
                    .prop("name", "radio")
                    .prop_signal("checked", state.host.signal_ref(clone!(embed_type => move |host| {
                        match host {
                            Some(host) => embed_type == partial_to_type(host),
                            None => false,
                        }
                    })))
                    .event(clone!(state, embed_type => move |_: events::Click| {
                        let mut host = state.host.lock_mut();
                        let is_selected = match &*host {
                            Some(host) => embed_type.clone() == partial_to_type(host),
                            None => false,
                        };
                        match is_selected {
                            true => {
                                *host = None;
                            },
                            false => {
                                let embed = type_to_partial(&embed_type);
                                *host = Some(embed);
                            },
                        };
                        drop(host);
                        state.on_embed_value_change();
                    }))
                }),
            ])
        })
    }

    fn render_host(self: &Rc<Self>, host: PartialEmbedHost) -> Dom {
        let state = self;
        let embed_type = partial_to_type(&host);
        html!("div", {
            .class("host")
            .child(html!("button-rect", {
                .prop("slot", "back")
                .prop("kind", "text")
                .prop("color", "blue")
                .child(html!("fa-icon", {
                    .prop("icon", "fa-light fa-chevron-left")
                }))
                .text(STR_BACK)
                .event(clone!(state => move |_: events::Click| {
                    state.delete_embed();
                }))
            }))
            .child(gap!(44))
            .child(html!("img-ui", {
                .prop("path", &format!(
                    "module/_common/edit/widgets/sidebar/embed/rectangle/{}.png",
                    embed_type.as_str()
                ))
            }))
            .child(gap!(16))
            .child(html!("input-wrapper" => HtmlElement, {
                .with_node!(wrapper => {
                    .prop("slot", "input")
                    .prop("label", &format!("Add a {} link", embed_type.display_name()))
                    .child(state.render_host_input(&host, wrapper))
                })
            }))
            .child(gap!(24))
            .child(html!("button-rect", {
                .prop("slot", "delete")
                .prop("kind", "text")
                .prop("color", "blue")
                .child(html!("fa-icon", {
                    .prop("icon", "fa-light fa-trash-can")
                }))
                .text(STR_DELETE)
                .event(clone!(state => move |_: events::Click| {
                    state.delete_embed();
                }))
            }))
            .apply(clone!(state => move |dom| {
                match state.render_host_specific_options(&host) {
                    Some(el) => dom.child(el),
                    None => dom,
                }
            }))
        })
    }

    fn render_host_input(self: &Rc<Self>, host: &PartialEmbedHost, wrapper: HtmlElement) -> Dom {
        let state = self;
        match &host {
            PartialEmbedHost::Youtube(youtube) => state.render_youtube_input(youtube, wrapper),
            PartialEmbedHost::Vimeo(vimeo) => state.render_vimeo_input(vimeo, wrapper),
            PartialEmbedHost::GoogleDoc(google_doc) => {
                state.render_google_doc_input(google_doc, wrapper)
            }
            PartialEmbedHost::GoogleForm(google_form) => {
                state.render_google_form_input(google_form, wrapper)
            }
            PartialEmbedHost::GoogleSheet(google_sheet) => {
                state.render_google_sheet_input(google_sheet, wrapper)
            }
            PartialEmbedHost::GoogleSlide(google_slide) => {
                state.render_google_slide_input(google_slide, wrapper)
            }
            PartialEmbedHost::Edpuzzle(_) => todo!(),
            PartialEmbedHost::Puzzel(_) => todo!(),
            PartialEmbedHost::Quizlet(quizlet) => state.render_quizlet_input(quizlet, wrapper),
            PartialEmbedHost::Thinglink(thinglink) => {
                state.render_thinglink_input(thinglink, wrapper)
            }
            PartialEmbedHost::Sutori(sutori) => state.render_sutori_input(sutori, wrapper),
        }
    }

    fn render_host_specific_options(self: &Rc<Self>, host: &PartialEmbedHost) -> Option<Dom> {
        let state = self;
        match &host {
            PartialEmbedHost::Youtube(youtube) => {
                Some(state.render_youtube_specific_options(youtube))
            }
            _ => None,
        }
    }
}

fn partial_to_type(partial: &PartialEmbedHost) -> EmbedHostType {
    match partial {
        PartialEmbedHost::Youtube(_) => EmbedHostType::Youtube,
        PartialEmbedHost::Vimeo(_) => EmbedHostType::Vimeo,
        PartialEmbedHost::GoogleDoc(_) => EmbedHostType::GoogleDoc,
        PartialEmbedHost::GoogleForm(_) => EmbedHostType::GoogleForm,
        PartialEmbedHost::GoogleSheet(_) => EmbedHostType::GoogleSheet,
        PartialEmbedHost::GoogleSlide(_) => EmbedHostType::GoogleSlide,
        PartialEmbedHost::Edpuzzle(_) => todo!(),
        PartialEmbedHost::Puzzel(_) => todo!(),
        PartialEmbedHost::Quizlet(_) => EmbedHostType::Quizlet,
        PartialEmbedHost::Thinglink(_) => EmbedHostType::Thinglink,
        PartialEmbedHost::Sutori(_) => todo!(),
    }
}
fn type_to_partial(partial: &EmbedHostType) -> PartialEmbedHost {
    match partial {
        EmbedHostType::Youtube => PartialEmbedHost::Youtube(Default::default()),
        EmbedHostType::Vimeo => PartialEmbedHost::Vimeo(Default::default()),
        EmbedHostType::GoogleDoc => PartialEmbedHost::GoogleDoc(Default::default()),
        EmbedHostType::GoogleForm => PartialEmbedHost::GoogleForm(Default::default()),
        EmbedHostType::GoogleSheet => PartialEmbedHost::GoogleSheet(Default::default()),
        EmbedHostType::GoogleSlide => PartialEmbedHost::GoogleSlide(Default::default()),
        // EmbedHostType::Edpuzzle => PartialEmbedHost::Edpuzzle(Default::default()),
        // EmbedHostType::Puzzel => PartialEmbedHost::Puzzel(Default::default()),
        EmbedHostType::Quizlet => PartialEmbedHost::Quizlet(Default::default()),
        EmbedHostType::Thinglink => PartialEmbedHost::Thinglink(Default::default()),
        // EmbedHostType::Sutori => PartialEmbedHost::Sutori(Default::default()),
    }
}
