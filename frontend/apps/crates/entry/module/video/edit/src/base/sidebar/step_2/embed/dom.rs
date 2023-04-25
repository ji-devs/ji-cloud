use std::rc::Rc;

use components::stickers::embed::types::PartialEmbedHost;
use dominator::{clone, html, Dom};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use super::{super::state::Step2, google_sheet::render_google_sheet, youtube::render_youtube};

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
    }
}
