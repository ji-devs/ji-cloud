use super::{state::*, video};
use components::{
    image::search::dom::render as render_image_search,
    tabs::{MenuTab, MenuTabKind},
    text_editor::dom::render_controls as render_text_editor,
};
use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use std::rc::Rc;

pub fn render(state: Rc<Step2>) -> Dom {
    let is_empty = map_ref! {
        let video = state.sidebar.base.video.signal_cloned()
            => {
            video.is_none()
        }
    };

    let tab = map_ref! {
        let tab = state.tab.signal_cloned(),
        let video = state.sidebar.base.video.signal_cloned()
            => {
            (video.is_none(), tab.kind())
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
        .future(tab.for_each(clone!(state => move |(is_empty, kind)| {
            state.sidebar.tab_kind.set(Some(kind));

            state.sidebar.base.can_continue_next.set_neq(match kind {
                MenuTabKind::Video => !is_empty,
                _ => true,
            });

            async move {}
        })))
        .child_signal(is_empty.map(clone!(state => move |is_empty| {
            Some(html!("menu-tabs", {
                .children(&mut [
                    render_tab(state.clone(), MenuTabKind::Video, true),
                    render_tab(state.clone(), MenuTabKind::Text, !is_empty),
                    render_tab(state.clone(), MenuTabKind::Image, !is_empty),
                    html!("module-sidebar-body", {
                        .property("slot", "body")
                        .child_signal(state.tab.signal_cloned().map(clone!(state => move |tab| {
                            match tab {
                                Tab::Video => {
                                    Some(video::render(Rc::clone(&state)))
                                },
                                Tab::Text => {
                                    Some(render_text_editor(state.sidebar.base.text_editor.clone()))
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
