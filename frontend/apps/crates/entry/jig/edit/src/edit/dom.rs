use std::rc::Rc;

use super::{
    super::edit::publish::Publish,
    state::State,
    iframe::dom::IframeDom,
    post_publish::dom::render as render_post_publish,
    selection::dom::SelectionDom,
    sidebar::dom::SidebarDom,
};
use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::{JigId, JigFocus};
use utils::prelude::*;

pub struct EditPage {}

impl EditPage {
    pub fn render(jig_id: JigId, jig_focus: JigFocus, route: JigEditRoute) -> Dom {
        let state = Rc::new(State::new(jig_id, jig_focus, route));

        html!("empty-fragment", {
            .child(html!("jig-edit-page", {
                /*
                 * this changes the url but does not preserve history
                 * commented out since it's misleading
                 * see the edit action for more comments
                 * we can navigate properly but then have full refresh
                 * might be worth popping/listening to location history
                 * but meh
                .future(module_id.signal().for_each(clone!(jig_id => move |module_id| {
                    let url:String = Route::Jig(JigRoute::Edit(jig_id, module_id)).into();

                    web_sys::window()
                        .unwrap_ji()
                        .history()
                        .unwrap_ji()
                        .push_state_with_url(&JsValue::NULL, "", Some(&url))
                        .unwrap_ji();
                    async {}
                })))
                */
                .apply(|dom| {
                    match jig_focus {
                        JigFocus::Modules => dom.child(SidebarDom::render(jig_id.clone(), state.clone())),
                        JigFocus::Resources => dom,
                    }
                })
                .child_signal(state.route.signal_cloned().map(clone!(state, jig_id => move |route| {
                        match route {
                        JigEditRoute::Landing => {
                            match jig_focus {
                                JigFocus::Modules => Some(SelectionDom::render()),
                                JigFocus::Resources => Some(Publish::render(Rc::clone(&state))),
                            }
                        },
                        JigEditRoute::Module(module_id) => {
                            Some(IframeDom::render(jig_id.clone(), module_id.clone()))
                        },
                        JigEditRoute::Publish => {
                            Some(Publish::render(Rc::clone(&state)))
                        }
                        JigEditRoute::PostPublish => {
                            Some(render_post_publish(jig_id.clone(), Rc::clone(&state)))
                        }
                    }
                })))
            }))
            .child_signal(state.play_jig.signal_cloned().map(clone!(state => move|play_jig| {
                play_jig.map(|settings| {
                    let close = clone!(state => move || {
                        state.play_jig.set(None);
                    });
                    PlayerPopup::render(
                        Rc::new(PlayerPopup::new(jig_id, settings.into(), PreviewPopupCallbacks::new(close))),
                        None
                    )
                })
            })))
        })
    }
}
