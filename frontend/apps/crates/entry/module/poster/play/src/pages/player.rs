use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal, always, self},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{Url, HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use utils::{
    iframe::*,
    components::module_page::*,
    signals::DefaultStringSignal,
    settings::SETTINGS,
};
use gloo_timers::future::TimeoutFuture;
use crate::{debug, data::*};
use std::future::Future;
use async_trait::async_trait;
use std::{
    pin,
    future,
    marker
};
use dominator::animation::{easing, Percentage, MutableAnimation, AnimatedMapBroadcaster};
use shared::{
    api::endpoints::{ApiEndpoint, self},
    domain,
    error,
    media::{image_id_to_key, MediaLibraryKind, MediaVariant},
};


use uuid::Uuid;
pub struct PlayerPage {
    game_state: Rc<GameState>,
}

impl PlayerPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let game_state = Rc::new(GameState::new(jig_id, module_id));
        Rc::new(Self { game_state })
    }
}

//Use the ModuleRenderer component by way of a trait
#[async_trait(?Send)]
impl ModuleRenderer for PlayerPage {
    type Data = raw::Poster;

    async fn load(_self:Rc<Self>) -> raw::Poster { 
        if let Some(raw_poster) = debug::settings().poster {
            raw_poster
        } else {
            log::info!("loading...");
            raw::Poster::load(_self.game_state.jig_id.clone(), _self.game_state.module_id.clone()).await
        }
    }

    fn render(_self: Rc<Self>, data: raw::Poster) -> ModuleRenderOutput {
        _self.game_state.set_from_loaded(data);
        ModuleRenderOutput::new_player( 
            html!("div", { 
                .child_signal(
                    _self.game_state.loaded.signal().map(clone!(_self => move |loaded| {
                        match loaded {
                            false => None,
                            true => Some(Self::render_loaded(_self.game_state.clone()))
                        }
                    }))
                )
            })
        )
    }

}

impl PlayerPage {
    fn render_loaded(game_state:Rc<GameState>) -> Dom {
        html!("h1", { .text("hello world") })
    }
}

