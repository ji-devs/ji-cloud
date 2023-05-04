use std::rc::Rc;

use components::stickers::embed::types::PartialEmbedHost;
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use super::{
    super::state::Step2, google_sheet::render_google_sheet, quizlet::render_quizlet,
    sutori::render_sutori, thinglink::render_thinglink, vimeo::render_vimeo,
    youtube::render_youtube,
};

fn option(
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

pub fn render(state: Rc<Step2>) -> Dom {
    html!("div", {
        .style("display", "grid")
        .style("row-gap", "38px")
        .child_signal(state.host.signal_cloned().map(clone!(state => move |embed| {
            embed.map(clone!(state => move |embed| {
                render_host(&state, embed)
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
                option(
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
                option(
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
                option(
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
                // option(
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
                // option(
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
                option(
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
                option(
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
                // option(
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

fn render_host(state: &Rc<Step2>, host: PartialEmbedHost) -> Dom {
    match &host {
        PartialEmbedHost::Youtube(youtube) => render_youtube(state, youtube),
        PartialEmbedHost::Vimeo(vimeo) => render_vimeo(state, vimeo),
        PartialEmbedHost::GoogleSheet(google_sheet) => render_google_sheet(state, google_sheet),
        PartialEmbedHost::Edpuzzle(_) => todo!(),
        PartialEmbedHost::Puzzel(_) => todo!(),
        PartialEmbedHost::Quizlet(quizlet) => render_quizlet(state, quizlet),
        PartialEmbedHost::Thinglink(thinglink) => render_thinglink(state, thinglink),
        PartialEmbedHost::Sutori(sutori) => render_sutori(state, sutori),
    }
}
