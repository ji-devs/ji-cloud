use std::rc::Rc;

use components::stickers::embed::types::PartialEmbedHost;
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use super::{
    super::state::Step2, google_sheet::render_google_sheet, quizlet::render_quizlet,
    sutori::render_sutori, thinglink::render_thinglink, youtube::render_youtube,
};

fn option(
    label: &str,
    on_select: Box<dyn Fn()>,
    selected: impl Signal<Item = bool> + 'static,
) -> Dom {
    html!("label", {
        .text(label)
        .style("display", "block")
        .child(html!("input", {
            .prop("type", "radio")
            .prop("name", "radio")
            .prop_signal("checked", selected)
            .event(move |_: events::Click| {
                (on_select)();
            })
        }))
    })
}

pub fn render(state: Rc<Step2>) -> Dom {
    html!("div", {
        .children(&mut [
            option(
                "youtube",
                Box::new(clone!(state => move|| {
                    match state.host.get_cloned() {
                        Some(PartialEmbedHost::Youtube(_)) => state.host.set(None),
                        _ => state.host.set(Some(PartialEmbedHost::Youtube(Default::default()))),
                    };
                    state.on_embed_value_change();
                })),
                state.host.signal_ref(|host| {
                    matches!(host, Some(PartialEmbedHost::Youtube(_)))
                })
            ),
            option(
                "google sheet",
                Box::new(clone!(state => move|| {
                    match state.host.get_cloned() {
                        Some(PartialEmbedHost::GoogleSheet(_)) => state.host.set(None),
                        _ => state.host.set(Some(PartialEmbedHost::GoogleSheet(Default::default()))),
                    };
                    state.on_embed_value_change();
                })),
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
                })
            ),
            option(
                "Sutori",
                Box::new(clone!(state => move|| {
                    match state.host.get_cloned() {
                        Some(PartialEmbedHost::Sutori(_)) => state.host.set(None),
                        _ => state.host.set(Some(PartialEmbedHost::Sutori(Default::default()))),
                    };
                    state.on_embed_value_change();
                })),
                state.host.signal_ref(|host| {
                    matches!(host, Some(PartialEmbedHost::Sutori(_)))
                })
            ),
        ])
        .child_signal(state.host.signal_cloned().map(clone!(state => move |embed| {
            embed.map(clone!(state => move |embed| {
                render_host(&state, embed)
            }))
        })))

    })
}

fn render_host(state: &Rc<Step2>, host: PartialEmbedHost) -> Dom {
    match &host {
        PartialEmbedHost::Youtube(youtube) => render_youtube(state, youtube),
        PartialEmbedHost::GoogleSheet(google_sheet) => render_google_sheet(state, google_sheet),
        PartialEmbedHost::Edpuzzle(_) => todo!(),
        PartialEmbedHost::Puzzel(_) => todo!(),
        PartialEmbedHost::Quizlet(quizlet) => render_quizlet(state, quizlet),
        PartialEmbedHost::Thinglink(thinglink) => render_thinglink(state, thinglink),
        PartialEmbedHost::Sutori(sutori) => render_sutori(state, sutori),
    }
}
