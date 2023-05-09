use std::rc::Rc;

use crate::stickers::embed::types::PartialEmbedHost;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use itertools::Itertools;
use utils::events;

use super::{EmbedHostType, EmbedSelect};

impl EmbedSelect {
    pub fn render(self: &Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .style("display", "grid")
            .style("row-gap", "38px")
            .child_signal(state.host.signal_cloned().map(clone!(state => move |embed| {
                embed.map(clone!(state => move |embed| {
                    state.render_host(embed)
                }))
            })))
            .child(html!("div", {
                .style("display", "grid")
                .style("grid-template-columns", "repeat(auto-fill, 56px)")
                .style("grid-auto-rows", "82px")
                .style("justify-content", "space-evenly")
                .style("column-gap", "48px")
                .style("row-gap", "24px")
                .children(state.render_options())
            }))
        })
    }

    fn render_options(self: &Rc<Self>) -> Vec<Dom> {
        let state = self;
        state
            .type_list
            .modules
            .iter()
            .map(clone!(state => move|module| {
                state.option(module)
            }))
            .collect_vec()
    }

    fn option(self: &Rc<Self>, embed_type: &EmbedHostType) -> Dom {
        let state = self;
        html!("label", {
            .style("display", "grid")
            .style("gap", "5px")
            .style("cursor", "pointer")
            .style("position", "relative")
            .children(&mut [
                html!("img-ui", {
                    .prop("path", &format!(
                        "module/_common/edit/widgets/sidebar/embed/{}.png",
                        embed_type.as_str()
                    ))
                    .style("background-color", "#ffffff")
                    .style("border-radius", "8px")
                    .style("overflow", "hidden")
                    .style("width", "56px")
                    .style("height", "56px")
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
                    .style("font-size", "13px")
                    .style("color", "var(--dark-gray-6)")
                    .style("margin", "0")
                    .style("translate", "calc(50% + -56px) 0")
                    .style("white-space", "nowrap")
                    .style("position", "absolute")
                    .style("top", "58px")
                    .style("left", "50%")
                    .style("translate", "-50%")
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
                    .style("display", "none")
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
        match &host {
            PartialEmbedHost::Youtube(youtube) => state.render_youtube(youtube),
            PartialEmbedHost::Vimeo(vimeo) => state.render_vimeo(vimeo),
            PartialEmbedHost::GoogleSheet(google_sheet) => state.render_google_sheet(google_sheet),
            PartialEmbedHost::Edpuzzle(_) => todo!(),
            PartialEmbedHost::Puzzel(_) => todo!(),
            PartialEmbedHost::Quizlet(quizlet) => state.render_quizlet(quizlet),
            PartialEmbedHost::Thinglink(thinglink) => state.render_thinglink(thinglink),
            PartialEmbedHost::Sutori(sutori) => state.render_sutori(sutori),
        }
    }
}

fn partial_to_type(partial: &PartialEmbedHost) -> EmbedHostType {
    match partial {
        PartialEmbedHost::Youtube(_) => EmbedHostType::Youtube,
        PartialEmbedHost::Vimeo(_) => EmbedHostType::Vimeo,
        PartialEmbedHost::GoogleSheet(_) => EmbedHostType::GoogleSheet,
        PartialEmbedHost::Edpuzzle(_) => EmbedHostType::Edpuzzle,
        PartialEmbedHost::Puzzel(_) => EmbedHostType::Puzzel,
        PartialEmbedHost::Quizlet(_) => EmbedHostType::Quizlet,
        PartialEmbedHost::Thinglink(_) => EmbedHostType::Thinglink,
        PartialEmbedHost::Sutori(_) => EmbedHostType::Sutori,
    }
}
fn type_to_partial(partial: &EmbedHostType) -> PartialEmbedHost {
    match partial {
        EmbedHostType::Youtube => PartialEmbedHost::Youtube(Default::default()),
        EmbedHostType::Vimeo => PartialEmbedHost::Vimeo(Default::default()),
        EmbedHostType::GoogleSheet => PartialEmbedHost::GoogleSheet(Default::default()),
        EmbedHostType::Edpuzzle => PartialEmbedHost::Edpuzzle(Default::default()),
        EmbedHostType::Puzzel => PartialEmbedHost::Puzzel(Default::default()),
        EmbedHostType::Quizlet => PartialEmbedHost::Quizlet(Default::default()),
        EmbedHostType::Thinglink => PartialEmbedHost::Thinglink(Default::default()),
        EmbedHostType::Sutori => PartialEmbedHost::Sutori(Default::default()),
    }
}
