use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable,ReadOnlyMutable, SignalExt, Signal},
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
};
use gloo_timers::future::TimeoutFuture;
use crate::{debug, data::{*, raw::*}};
use std::future::Future;
use async_trait::async_trait;
use std::{
    pin,
    future,
    marker
};
use dominator::animation::{easing, Percentage, MutableAnimation, AnimatedMapBroadcaster};
pub struct PlayerPage {
    state: GameState,
}

impl PlayerPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let state = GameState::new(jig_id, module_id);
        Rc::new(Self { state })
    }
}

//Use the ModuleRenderer component by way of a trait
#[async_trait(?Send)]
impl ModuleRenderer for PlayerPage {
    type Data = GameStateRaw;

    async fn load(_self:Rc<Self>) -> GameStateRaw { 
        if let Some(raw_state) = debug::settings().state {
            raw_state
        } else {
            log::info!("loading...");
            GameStateRaw::load().await
        }
    }

    fn render(_self: Rc<Self>, data: GameStateRaw) -> Dom {
        _self.state.set_from_loaded(data);
        html!("div", {
            .child_signal(_self.state.mode.signal_ref(clone!(_self => move |mode| {
                mode.map(clone!(_self => move |_| {
                    match _self.state.mode_state.borrow().as_ref() {
                        None => panic!("can't render player without state!"),
                        Some(mode) => match mode {
                            ModeState::Duplicate(state) => {
                                DuplicatePlayer::render(DuplicatePlayer::new(state.clone()))
                            },
                            _ => unimplemented!("todo - other modes!")
                        }
                    }
                }))
            })))
        })
    }
}


pub struct DuplicatePlayer {
    state: Rc<DuplicateState>,
}

impl DuplicatePlayer {
    pub fn new(state:Rc<DuplicateState>) -> Rc<Self> {
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
            .with_data_id!("hover-cards", {
                .children_signal_vec(Self::hover_cards_dom_signal(_self.clone()))
            })

            .with_data_id!("found-cards", {
                .children_signal_vec(Self::found_cards_dom_signal(_self.clone()))
            })
        })
    }

    fn found_cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.found_cards
            .signal_vec_cloned()
            .map(clone!(_self => move |card| {
                FoundCardDom::render(FoundCardDom::new(_self.state.clone(), card))
            }))
    }

    fn hover_cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.hover_cards
            .signal_vec_cloned()
            .map(clone!(_self => move |card| {
                HoverCardDom::render(HoverCardDom::new(_self.state.clone(), card))
            }))
    }
    fn game_cards_dom_signal(_self: Rc<Self>) -> impl SignalVec<Item = Dom> {
        _self.state.game_cards
            .signal_vec_cloned()
            //this allows us to hide the visuals of empty cards, but it gets weird
            //.filter_signal_cloned(|card| card.text.signal_ref(|text| !text.is_empty()))
            .map(clone!(_self => move |(card)| {
                GameCardDom::render(GameCardDom::new(_self.state.clone(), card))
            }))
    }
}

pub struct GameCardDom {
    pub state: Rc<DuplicateState>,
    pub is_hover:Mutable<bool>,
    pub card: GameCard,
}

impl GameCardDom {
    pub fn new(state:Rc<DuplicateState>, card: GameCard) -> Rc<Self> {
        Rc::new(Self {
            state,
            is_hover: Mutable::new(false),
            card 
        })
    }

    fn is_showing(&self) -> impl Signal<Item = bool> {
        let my_id = self.card.id;

        self.state.flip_state.signal_cloned().map(move |flip_state| {
            match flip_state {
                FlipState::None => false,
                FlipState::One(flip_card) => flip_card.id == my_id,
                FlipState::Two((flip_card_1, flip_card_2)) => {
                    flip_card_1.id == my_id || flip_card_2.id == my_id
                }
            }
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::card(), {
            //maybe something better than opacity... gotta take up grid space though
            .class_signal("opacity-0", _self.card.found.signal())
            .class_signal("flip-card-clicked", _self.is_showing().map(|x| !x))
            .with_node!(element => {
                .event(clone!(_self, element => move |evt:events::Click| {
                    _self.state.card_click(_self.card.id, element.clone());
                }))
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
            .with_data_id!("text-contents", {
                .text(&_self.card.text)
            })
        })
    }
}


pub struct HoverCardDom {
    pub state: Rc<DuplicateState>,
    pub card: HoverCard,
    pub animation: MutableAnimation
}

impl HoverCardDom {
    pub fn new(state:Rc<DuplicateState>, card: HoverCard) -> Rc<Self> {
        let animation = MutableAnimation::new(3000.0);
        animation.animate_to(Percentage::new(1.0));

        //animation.play();

        Rc::new(Self {
            state,
            card,
            animation
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        let origin_x:f64 = _self.card.origin_x;
        let origin_y:f64 = _self.card.origin_y;
        let dest_x:f64 = _self.card.dest_x;
        let dest_y:f64 = _self.card.dest_y;

        elem!(templates::hover_card(), {
            .future(clone!(_self => async move {
                _self.animation
                    .signal()
                    .wait_for(Percentage::new(1.0))
                    .await;
                _self.state.move_animation_finished(&_self.card);
            }))
            .style_signal("top", _self.animation.signal()
                .map(move |t| easing::in_out(t, easing::cubic))
                .map(move |t| Some(format!("{}rem", t.range_inclusive(origin_y, dest_y)))))
            .style_signal("left", _self.animation.signal()
                .map(move |t| easing::in_out(t, easing::cubic))
                .map(move |t| Some(format!("{}rem", t.range_inclusive(origin_x, dest_x)))))
            .with_data_id!("text-contents", {
                .text(&_self.card.text)
            })
        })
    }
}


pub struct FoundCardDom {
    pub state: Rc<DuplicateState>,
    pub card: FoundCard,
}

impl FoundCardDom {
    pub fn new(state:Rc<DuplicateState>, card: FoundCard) -> Rc<Self> {
        Rc::new(Self {
            state,
            card 
        })
    }

    pub fn render(_self: Rc<Self>) -> Dom { 
        elem!(templates::hover_card(), {
            .style("top", format!("{}rem", _self.card.y))
            .style("left", format!("{}rem", _self.card.x))
            .with_data_id!("text-contents", {
                .text(&_self.card.text)
            })
        })
    }
}
