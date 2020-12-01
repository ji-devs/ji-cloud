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
    game_state: GameState,
}

impl PlayerPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let game_state = GameState::new(jig_id, module_id);
        Rc::new(Self { game_state })
    }
}

//Use the ModuleRenderer component by way of a trait
#[async_trait(?Send)]
impl ModuleRenderer for PlayerPage {
    type Data = raw::GameState;

    async fn load(_self:Rc<Self>) -> raw::GameState { 
        if let Some(raw_state) = debug::settings().state {
            raw_state
        } else {
            log::info!("loading...");
            raw::GameState::load(_self.game_state.jig_id.clone(), _self.game_state.module_id.clone()).await
        }
    }

    fn render(_self: Rc<Self>, data: raw::GameState) -> ModuleRenderOutput {
        _self.game_state.set_from_loaded(data);
        ModuleRenderOutput::new_player( 
            html!("div", {
                .child_signal(_self.game_state.mode.signal_ref(clone!(_self => move |mode| {
                    mode.map(clone!(_self => move |mode| {
                        if let Some(mode) = mode {
                            let state = Rc::new(_self.game_state.state.borrow_mut().take().unwrap_throw());

                            match mode { 
                                GameMode::Duplicate => {
                                    DuplicatePlayer::render(DuplicatePlayer::new(state))
                                },
                                _ => unimplemented!("todo - other modes!")
                            }
                        } else {
                            panic!("no game mode!");
                        }
                    }))
                })))
            })
        )
    }
}


pub struct DuplicatePlayer {
    state: Rc<BaseGameState>,
}

impl DuplicatePlayer {
    pub fn new(state:Rc<BaseGameState>) -> Rc<Self> {
        Rc::new(Self { state })
    }

    pub fn render(_self:Rc<DuplicatePlayer>) -> Dom {
        elem!(templates::player(), {
            .class(format!("memory-theme-{}", _self.state.theme_id))
            .future(_self.state.flip_state.signal_cloned().for_each(clone!(_self => move |flip_state| {
                clone!(_self => async move {
                    match flip_state {
                        FlipState::Two((card_1, card_2)) => {
                            _self.state.evaluate(card_1, card_2).await;
                        },
                        _ => {}
                    }
                })
            })))
            .with_data_id!("game-cards", {
                .class(format!("memory-grid-{}", _self.state.grid_number()))
                .children_signal_vec(Self::game_cards_dom_signal(_self.clone()))
            })
        })
    }

    fn game_cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.game_cards
            .signal_vec_cloned()
            .map(clone!(_self => move |(card)| {
                CardDom::render(CardDom::new(_self.state.clone(), card))
            }))
    }
}

pub struct CardDom {
    pub state: Rc<BaseGameState>,
    pub is_hover:Mutable<bool>,
    pub card: Card,
    pub transition: Mutable<Option<CardTransition>>
}

pub struct CardTransition {
    pub animation: MutableAnimation,
    pub dest_x: f64,
    pub dest_y: f64,
    pub dest_rot: f64, 
    pub side: Side,
}

impl CardTransition {
    pub fn new(element:&HtmlElement, found_index: usize, side:Side) -> Self {

        let animation = MutableAnimation::new(3000.0);
        animation.animate_to(Percentage::new(1.0));

        let (origin_x, origin_y) = utils::resize::ModuleBounds::get_element_pos_rem(element);
        let mut dest_x = if side == Side::Left { 0.0 } else { 10.0 };
        dest_x -= origin_x;

        let start_y = 5.0;
        let line_offset = 20.0;
        let mut dest_y = start_y + (found_index as f64 * line_offset);
        dest_y -= origin_y;

        let dest_rot:f64 = if side == Side::Right { -20.0 } else { 0.0 };
        Self {
            animation,
            dest_x,
            dest_y,
            dest_rot,
            side,
        }
    }

    fn transform_signal(&self) -> impl Signal<Item = String> {
        let dest_x = self.dest_x; 
        let dest_y = self.dest_y; 
        let dest_rot = self.dest_rot;
        self.animation.signal()
            .map(move |t| easing::in_out(t, easing::cubic))
            .map(move |t| (
                t.range_inclusive(0.0, dest_x),
                t.range_inclusive(0.0, dest_y),
                t.range_inclusive(0.0, dest_rot),
            ))
            .map(move |(x, y, rot)| {
                format!("translate({}rem, {}rem) rotateZ({}deg)", x, y, rot)
            })
    }
}
impl CardDom {
    pub fn new(state:Rc<BaseGameState>, card: Card) -> Rc<Self> {
        Rc::new(Self {
            state,
            is_hover: Mutable::new(false),
            card ,
            transition: Mutable::new(None),
        })
    }

    fn is_showing(&self) -> impl Signal<Item = bool> {
        let my_id = self.card.id;

        map_ref! {
            let flip_state = self.state.flip_state.signal_cloned(),
            let found = self.card.found.signal()
            => move {
                if found.is_some() {
                    true
                } else {
                    match flip_state {
                        FlipState::None => false,
                        FlipState::One(id) => *id == my_id,
                        FlipState::Two((id_1, id_2)) => {
                            *id_1 == my_id || *id_2 == my_id
                        }
                    }
                }
            }
        }
    }

    fn transform_signal(&self) -> impl Signal<Item = String> {
        self.transition.signal_ref(|transition| {
            DefaultStringSignal::new(
                "none".to_string(),
                transition.as_ref().map(|t| t.transform_signal())
            )
        })
        .flatten()
    }
    fn depth_signal(&self, side: Side) -> impl Signal<Item = &'static str> {

        self.transition.signal_ref(clone!(side => move |t| {
            match t {
                None => "0",
                Some(_) => {
                    if side == Side::Left {
                        "10"
                    } else {
                        "9"
                    }
                }
            }
        }))
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::card(), {
            .with_node!(element => {
                .future(_self.card.found.signal().for_each(clone!(_self,element => move |found| {
                    if let Some(found_index) = found {
                        _self.transition.set(Some(CardTransition::new(&element, found_index, _self.card.side)));
                    }
                    async {}
                })))
            })


            .event(clone!(_self => move |evt:events::Click| {
                _self.state.card_click(_self.card.id);
            }))
            .style_signal("z-index", _self.depth_signal(_self.card.side))
            .style_signal("transform", _self.transform_signal())
            .class_signal("flip-card-clicked", _self.is_showing().map(|x| !x))
            .with_node!(element => {
                .event(clone!(_self => move |evt:events::MouseEnter| {
                    _self.is_hover.set(true);
                }))
                .event_preventable(clone!(_self => move |evt:events::MouseLeave| {
                    if let Some(target) = evt.target() {
                        if target == element.clone().unchecked_into() {
                            _self.is_hover.set(false);
                        } else {
                            evt.prevent_default();
                        }
                    }
                }))
            })
            .apply(|dom| {
                match &_self.card.media {
                    Media::Text(text) => {
                        apply_methods!(dom, {
                            .with_data_id!("text-contents", {
                                .text(text)
                            })
                            .with_data_id!("image", {
                                .class("hidden")
                            })
                        })
                    },
                    Media::Image(id) => {
                        apply_methods!(dom, {
                            .with_data_id!("image", {
                                .property("src", {
                                    id.as_ref().map(|id| utils::path::library_image(MediaLibraryKind::Global, MediaVariant::Resized, id)) 
                                        .unwrap_or("".to_string())
                                })
                            })
                            .with_data_id!("text-contents", {
                                .class("hidden")
                            })
                        })
                    },
                    _ => unimplemented!("don't know how to render media type!")
                }
            })
        })
    }
}
