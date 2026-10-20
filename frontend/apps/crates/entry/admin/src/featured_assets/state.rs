use components::asset_search_bar::{AssetSearchBar, SearchSelected};
use dominator_helpers::futures::AsyncLoader;
use futures_signals::signal::Mutable;
use futures_signals::signal_vec::MutableVec;
use shared::domain::asset::{Asset, AssetId};
use std::rc::Rc;
use utils::{drag::Drag, unwrap::UnwrapJiExt};
use web_sys::HtmlElement;

#[derive(Clone, Copy)]
pub enum FeaturedKind {
    Jigs,
    Playlists,
}

impl FeaturedKind {
    pub(super) fn accepts(self, asset: &Asset) -> bool {
        matches!(
            (self, asset),
            (Self::Jigs, Asset::Jig(_)) | (Self::Playlists, Asset::Playlist(_))
        )
    }

    pub(super) fn search_kind(self) -> &'static str {
        match self {
            Self::Jigs => "jig",
            Self::Playlists => "playlist",
        }
    }

    pub(super) fn count_prop(self) -> &'static str {
        match self {
            Self::Jigs => "jigCount",
            Self::Playlists => "playlistCount",
        }
    }

    pub(super) fn instructions(self) -> &'static str {
        match self {
            Self::Jigs => "Drag in JIGs",
            Self::Playlists => "Drag in playlists",
        }
    }

    pub(super) fn duplicate_error(self) -> &'static str {
        match self {
            Self::Jigs => "Jig already in featured list",
            Self::Playlists => "Playlist already in featured list",
        }
    }

    pub(super) fn invalid_error(self) -> &'static str {
        match self {
            Self::Jigs => "Featured JIGs must contain only JIGs",
            Self::Playlists => "Featured playlists must contain only playlists",
        }
    }
}

pub struct FeaturedAssets {
    pub(super) kind: FeaturedKind,
    pub(super) search_bar: Rc<AssetSearchBar>,
    pub(super) loader: AsyncLoader,
    pub(super) search_results: MutableVec<Rc<Asset>>,
    pub(super) next_page: Mutable<u32>,
    pub(super) active_query: Mutable<String>,
    pub(super) total_count: Mutable<u32>,
    pub(super) drag: Mutable<Option<Rc<Drag<Asset>>>>,
    pub(super) play_asset: Mutable<Option<AssetId>>,
    pub(super) featured_assets: MutableVec<Asset>,
    pub(super) dragging_over_drop_section: Mutable<bool>,
}

impl FeaturedAssets {
    pub fn new(kind: FeaturedKind) -> Rc<Self> {
        Rc::new(Self {
            kind,
            search_bar: AssetSearchBar::new_with_search_selected(SearchSelected {
                rated_only: Mutable::new(false),
                ..Default::default()
            }),
            loader: AsyncLoader::new(),
            search_results: Default::default(),
            next_page: Default::default(),
            active_query: Default::default(),
            total_count: Default::default(),
            drag: Default::default(),
            play_asset: Default::default(),
            featured_assets: MutableVec::new(),
            dragging_over_drop_section: Mutable::new(false),
        })
    }

    pub(super) fn on_pointer_down(
        self: &Rc<Self>,
        elem: &HtmlElement,
        x: i32,
        y: i32,
        asset: &Rc<Asset>,
    ) {
        let drag = Drag::new_anchor_element_resize(x, y, elem, true, (**asset).clone());
        self.drag.set(Some(Rc::new(drag)));
    }

    pub(super) fn on_pointer_move(self: &Rc<Self>, drag: &Rc<Drag<Asset>>, x: i32, y: i32) {
        drag.update(x, y);
    }

    pub(super) fn on_pointer_up(self: &Rc<Self>, drag: &Rc<Drag<Asset>>, x: i32, y: i32) {
        let data = serde_json::to_string(&drag.data).unwrap_ji();
        drag.trigger_drop_event(x, y, &data);
        self.stop_drag();
    }

    pub(super) fn stop_drag(self: &Rc<Self>) {
        self.drag.set(None);
    }
}
