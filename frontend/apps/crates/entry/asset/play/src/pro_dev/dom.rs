use super::player_main::PlayerMain;
use dominator::DomBuilder;
use std::rc::Rc;
use utils::component::Component;
use web_sys::ShadowRoot;

use super::state::ProDevPlayer;

impl Component<ProDevPlayer> for Rc<ProDevPlayer> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        self.load_data();
        dom.child(PlayerMain::new(self).render())
    }
}
