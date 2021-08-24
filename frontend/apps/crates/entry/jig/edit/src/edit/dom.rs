use std::rc::Rc;

use super::{
    iframe::dom::IframeDom, publish::dom::render as render_publish, post_publish::dom::render as render_post_publish, selection::dom::SelectionDom,
    sidebar::dom::SidebarDom,
};
use components::player_popup::{PlayerPopup, PreviewPopupCallbacks};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::JigId;
use utils::prelude::*;
use super::state::State;

pub struct EditPage {}

impl EditPage {
    pub fn render(jig_id: JigId, route: JigEditRoute) -> Dom {
        let state = Rc::new(State::new(jig_id, route));

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
                .child(SidebarDom::render(jig_id.clone(), state.clone()))
                .child_signal(state.route.signal_cloned().map(clone!(state, jig_id => move |route| {
                    match route {
                        JigEditRoute::Landing => {
                            Some(SelectionDom::render())
                        },
                        JigEditRoute::Module(module_id) => {
                            Some(IframeDom::render(jig_id.clone(), module_id.clone()))
                        },
                        JigEditRoute::Publish => {
                            Some(render_publish(Rc::clone(&state)))
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
                        Rc::new(PlayerPopup::new(jig_id, settings, PreviewPopupCallbacks::new(close))),
                        None
                    )
                })
            })))
        })
    }
}
