use std::rc::Rc;

use crate::stickers::embed::types::PartialEmbedHost;
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use super::EmbedSelect;

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
                .children(&mut [
                    state.option(
                        "Youtube",
                        Box::new(clone!(state => move|| {
                            match state.host.get_cloned() {
                                Some(PartialEmbedHost::Youtube(_)) => state.host.set(None),
                                _ => state.host.set(Some(PartialEmbedHost::Youtube(Default::default()))),
                            };
                            state.on_embed_value_change();
                        })),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Youtube(_)))
                        }),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Youtube(_)))
                        })
                    ),
                    state.option(
                        "Vimeo",
                        Box::new(clone!(state => move|| {
                            match state.host.get_cloned() {
                                Some(PartialEmbedHost::Vimeo(_)) => state.host.set(None),
                                _ => state.host.set(Some(PartialEmbedHost::Vimeo(Default::default()))),
                            };
                            state.on_embed_value_change();
                        })),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Vimeo(_)))
                        }),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Vimeo(_)))
                        })
                    ),
                    state.option(
                        "Google Sheets",
                        Box::new(clone!(state => move|| {
                            match state.host.get_cloned() {
                                Some(PartialEmbedHost::GoogleSheet(_)) => state.host.set(None),
                                _ => state.host.set(Some(PartialEmbedHost::GoogleSheet(Default::default()))),
                            };
                            state.on_embed_value_change();
                        })),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::GoogleSheet(_)))
                        }),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::GoogleSheet(_)))
                        })
                    ),
                    // state.option(
                    //     "Edpuzzle",
                    //     Box::new(clone!(state => move|| {
                    //         match state.host.get_cloned() {
                    //             Some(PartialEmbedHost::Edpuzzle(_)) => state.host.set(None),
                    //             _ => state.host.set(Some(PartialEmbedHost::Edpuzzle(Default::default()))),
                    //         };
                    //         state.on_embed_value_change();
                    //     })),
                    //     state.host.signal_ref(|host| {
                    //         matches!(host, Some(PartialEmbedHost::Edpuzzle(_)))
                    //     })
                    // ),
                    // state.option(
                    //     "Puzzel",
                    //     Box::new(clone!(state => move|| {
                    //         match state.host.get_cloned() {
                    //             Some(PartialEmbedHost::Puzzel(_)) => state.host.set(None),
                    //             _ => state.host.set(Some(PartialEmbedHost::Puzzel(Default::default()))),
                    //         };
                    //         state.on_embed_value_change();
                    //     })),
                    //     state.host.signal_ref(|host| {
                    //         matches!(host, Some(PartialEmbedHost::Puzzel(_)))
                    //     })
                    // ),
                    state.option(
                        "Quizlet",
                        Box::new(clone!(state => move|| {
                            match state.host.get_cloned() {
                                Some(PartialEmbedHost::Quizlet(_)) => state.host.set(None),
                                _ => state.host.set(Some(PartialEmbedHost::Quizlet(Default::default()))),
                            };
                            state.on_embed_value_change();
                        })),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Quizlet(_)))
                        }),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Quizlet(_)))
                        })
                    ),
                    state.option(
                        "Thinglink",
                        Box::new(clone!(state => move|| {
                            match state.host.get_cloned() {
                                Some(PartialEmbedHost::Thinglink(_)) => state.host.set(None),
                                _ => state.host.set(Some(PartialEmbedHost::Thinglink(Default::default()))),
                            };
                            state.on_embed_value_change();
                        })),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Thinglink(_)))
                        }),
                        state.host.signal_ref(|host| {
                            matches!(host, Some(PartialEmbedHost::Thinglink(_)))
                        })
                    ),
                    // state.option(
                    //     "Sutori",
                    //     Box::new(clone!(state => move|| {
                    //         match state.host.get_cloned() {
                    //             Some(PartialEmbedHost::Sutori(_)) => state.host.set(None),
                    //             _ => state.host.set(Some(PartialEmbedHost::Sutori(Default::default()))),
                    //         };
                    //         state.on_embed_value_change();
                    //     })),
                    //     state.host.signal_ref(|host| {
                    //         matches!(host, Some(PartialEmbedHost::Sutori(_)))
                    //     }),
                    //     state.host.signal_ref(|host| {
                    //         matches!(host, Some(PartialEmbedHost::Sutori(_)))
                    //     })
                    // ),
                ])
            }))
        })
    }

    fn option(
        self: &Rc<Self>,
        label: &str,
        on_select: Box<dyn Fn()>,
        selected_a: impl Signal<Item = bool> + 'static,
        selected_b: impl Signal<Item = bool> + 'static,
    ) -> Dom {
        html!("label", {
            .style("display", "grid")
            .style("gap", "5px")
            .style("cursor", "pointer")
            .style("position", "relative")
            .children(&mut [
                html!("img-ui", {
                    .prop("path", "...")
                    .style("background-color", "#ffffff")
                    .style("border-radius", "8px")
                    .style("overflow", "hidden")
                    .style("width", "56px")
                    .style("height", "56px")
                    .style_signal("outline", selected_a.map(|selected| {
                        match selected {
                            true => "solid 3px var(--light-blue-5)",
                            false => "none",
                        }
                    }))
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
                    .text(label)
                }),
                html!("input", {
                    .prop("type", "radio")
                    .prop("name", "radio")
                    .prop_signal("checked", selected_b)
                    .style("display", "none")
                    .event(move |_: events::Click| {
                        (on_select)();
                    })
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
