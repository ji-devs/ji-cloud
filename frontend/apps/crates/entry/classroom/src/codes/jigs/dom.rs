use std::rc::Rc;

use components::asset_card::render_asset_card;
use dominator::{html, DomBuilder};
use futures_signals::signal_vec::SignalVecExt;
use shared::domain::asset::Asset;
use utils::{
    component::Component,
    link,
    routes::{ClassroomCodesRoute, ClassroomRoute, Route},
};
use web_sys::ShadowRoot;

use super::Jigs;

impl Component<Jigs> for Rc<Jigs> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("div", {
            .children_signal_vec(state.jigs.signal_vec_cloned().map(|jig| {
                let asset = Asset::Jig(jig.clone());
                link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodes(jig.id))), {
                    .child(render_asset_card(&asset, Default::default()))
                })
            }))
        }))
    }
}
