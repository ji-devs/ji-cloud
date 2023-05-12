use super::state::*;
use components::{
    image::search::dom::render as render_image_search,
    module::_groups::design::edit::embed::select::{EmbedSelect, EmbedSelectList},
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use utils::component::Component;
use std::rc::Rc;

pub fn render(state: Rc<Step2>) -> Dom {
    let is_empty = map_ref! {
        let embed = state.sidebar.base.embed.signal_cloned()
            => {
            embed.is_none()
        }
    };

    let tab = map_ref! {
        let tab = state.tab.signal_cloned(),
        let embed = state.sidebar.base.embed.signal_cloned()
            => {
            (embed.is_none(), tab.kind())
        }
    };

    state
        .sidebar
        .base
        .continue_next_fn
        .set(Some(Rc::new(clone!(state => move || {
            if let Some(kind) = state.next_kind() {
                state.tab.set(Tab::new(state.sidebar.base.clone(), kind));
                true
            } else {
                false
            }
        }))));

    html!("empty-fragment", {
        .style("display", "contents")
        .future(tab.for_each(clone!(state => move |(is_empty, kind)| {
            state.sidebar.tab_kind.set(Some(kind));

            state.sidebar.base.can_continue_next.set_neq(match kind {
                MenuTabKind::Embed => !is_empty,
                _ => true,
            });

            async move {}
        })))
        .child_signal(is_empty.map(clone!(state => move |is_empty| {
            Some(html!("menu-tabs", {
                .children(&mut [
                    render_tab(state.clone(), MenuTabKind::Embed, true),
                    render_tab(state.clone(), MenuTabKind::Text, !is_empty),
                    render_tab(state.clone(), MenuTabKind::Image, !is_empty),
                    html!("module-sidebar-body", {
                        .prop("slot", "body")
                        .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                            match tab {
                                Tab::Embed => {
                                    let base = &state.sidebar.base;
                                    Some(EmbedSelect::new(EmbedSelectList::all(), &base.stickers, base.embed.clone()).render())
                                },
                                Tab::Text => {
                                    Some(state.sidebar.base.text_editor.render_controls())
                                },
                                Tab::Image(state) => {
                                    Some(render_image_search(state, None))
                                },
                            }
                        })))
                    }),
                ])
            }))
        })))
    })
}

fn render_tab(state: Rc<Step2>, tab_kind: MenuTabKind, enabled: bool) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            enabled,
            clone!(state => move || state.tab.signal_ref(clone!(tab_kind => move |curr| {
                curr.kind() == tab_kind
            }))),
            clone!(state, tab_kind => move || {
                state.tab.set(Tab::new(state.sidebar.base.clone(), tab_kind));
            }),
        ),
        Some("tabs"),
    )
}
