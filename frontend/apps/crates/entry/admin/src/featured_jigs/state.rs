use components::asset_search_bar::{AssetSearchBar, SearchSelected};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::jig::{JigId, JigResponse};
use std::rc::Rc;
use utils::{drag::Drag, unwrap::UnwrapJiExt};
use web_sys::HtmlElement;

pub struct FeaturedJigs {
    pub search_bar: Rc<AssetSearchBar>,
    pub loader: AsyncLoader,
    pub search_results: MutableVec<Rc<JigResponse>>,
    pub next_page: Mutable<u32>,
    pub active_query: Mutable<String>,
    pub total_jig_count: Mutable<u32>,
    pub drag: Mutable<Option<Rc<Drag<JigResponse>>>>,
    pub play_jig: Mutable<Option<JigId>>,
    pub featured_jigs: MutableVec<JigResponse>,
    pub dragging_over_drop_section: Mutable<bool>,
}
impl FeaturedJigs {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            search_bar: AssetSearchBar::new_with_search_selected(SearchSelected {
                rated_only: Mutable::new(false),
                ..Default::default()
            }),
            loader: AsyncLoader::new(),
            search_results: Default::default(),
            next_page: Default::default(),
            active_query: Default::default(),
            total_jig_count: Default::default(),
            drag: Default::default(),
            play_jig: Default::default(),
            featured_jigs: MutableVec::new(),
            dragging_over_drop_section: Mutable::new(false),
        })
    }

    pub fn on_pointer_down(
        self: &Rc<Self>,
        elem: &HtmlElement,
        x: i32,
        y: i32,
        jig: &Rc<JigResponse>,
    ) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, (**jig).clone().into());
        self.drag.set(Some(Rc::new(drag)));
    }

    pub fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<JigResponse>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub fn on_pointer_up(self: &Rc<Self>, drag: &Rc<Drag<JigResponse>>, x: i32, y: i32) {
        let data = serde_json::to_string(&drag.data).unwrap_ji();
        drag.trigger_drop_event(x, y, &data);
        self.stop_drag();
    }

    pub fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
