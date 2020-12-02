use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, ReadOnlyMutable},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlIFrameElement,HtmlTextAreaElement};
use dominator::{DomBuilder, Dom, html, events, with_node, clone, apply_methods};
use dominator_helpers::{elem,dynamic_class_signal ,with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use utils::components::module_page::*;
use async_trait::async_trait;
use super::choose_mode;
use wasm_bindgen::JsCast;
use utils::components::image::data::*;
use shared::media::{image_id_to_key, MediaLibraryKind, MediaVariant};
pub struct Main {
    state: Rc<State>, 
    game_mode: GameMode,
}
use utils::settings::SETTINGS;
use shared::domain::jig::ModuleKind;
use utils::{
    iframe::*,
    routes::{Route, ModuleRoute}
};

#[derive(Clone, PartialEq, Debug)]
enum MainMode {
    EmptyCards,
    Pairs,
    Iframe
}
impl Main {
    pub fn new(state: Rc<State>, game_mode:GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state, 
            game_mode,
        })
    }

    fn main_mode_signal(&self) -> impl Signal<Item = MainMode> {
        map_ref! {
            let step = self.state.step.signal(),
            let cards_hide = self.state.cards_hide_signal()
            => {
                let step = *step;
                let cards_hide = *cards_hide;
                
                if step == Step::Four {
                    MainMode::Iframe
                } else if cards_hide {
                    MainMode::EmptyCards
                } else {
                    MainMode::Pairs
                }
            }
        }
    }
    pub fn render(_self: Rc<Self>) -> Dom {
        html!("div", {

            .dynamic_class_signal!(_self.state.theme_id.signal_ref(|id| {
                Some(format!("memory-theme-{}", id))
            }))
            .child_signal(_self.main_mode_signal().map(clone!(_self => move |main_mode| {
                log::info!("{:?}", main_mode);

                Some(
                    match main_mode {
                        MainMode::EmptyCards => elem!(templates::main_empty(), {}),
                        MainMode::Pairs => Self::render_pairs(_self.clone()),
                        MainMode::Iframe => Self::render_iframe(_self.clone()),
                    }
                )
            })))
        })

    }
    fn iframe_url(&self) -> String {
        let route:String = Route::Module(ModuleRoute::Play(ModuleKind::MemoryGame, self.state.jig_id.clone(), self.state.module_id.clone())).into();

        let url = unsafe {
            SETTINGS.get_unchecked()
            .remote_target
            .spa_iframe(&route)
        };

        format!("{}?iframe_data=true", url)
    }
    fn render_iframe(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::main_iframe(), {
            .with_data_id!("iframe" => HtmlIFrameElement, {
                .property("src", &_self.iframe_url())
                .with_node!(elem => {
                    .global_event(clone!(_self => move |evt:dominator_helpers::events::Message| {

                        if let Ok(_) = evt.try_serde_data::<IframeInit<()>>() {
                            //Iframe is ready and sent us a message, let's send one back!
                            let data = _self.state.to_raw(); 
                            let msg:IframeInit<raw::GameData> = IframeInit::new(data); 
                            let window = elem.content_window().unwrap_throw();
                            window.post_message(&msg.into(), &_self.iframe_url());
                        } else {
                            log::info!("hmmm got other iframe message...");
                        }
                    }))
                    
                })

            })
        })
    }
    fn render_pairs(_self: Rc<Self>) -> Dom {
        let state = _self.state.clone();
        elem!(templates::main_pairs(), {
            .with_data_id!("cards", {
                .children_signal_vec(
                    state.pairs
                        .signal_vec_cloned()
                        .enumerate()
                        .map(clone!(state => move |(index, pair)| {
                            PairDom::render(
                                PairDom::new(state.clone(), index, pair.0, pair.1)
                            )
                        }))
                )
            })
        })
    }
}

struct PairDom {
    state: Rc<State>,
    index: ReadOnlyMutable<Option<usize>>,
    left: Card,
    right: Card
}



impl PairDom {
    fn new(state:Rc<State>, index: ReadOnlyMutable<Option<usize>>, left: Card, right: Card) -> Rc<Self> {
        Rc::new(Self { state, index, left, right })
    }

    fn dom_signal(_self:Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        let state = _self.state.clone();
        let left_data = _self.left.data.clone();
        let right_data = _self.right.data.clone();
        
        map_ref! {
            let game_mode = _self.state.game_mode.signal(),
            let index = _self.index.signal(),
            let left_mode = _self.left.mode.signal_cloned(),
            let right_mode = _self.right.mode.signal_cloned(),
            let is_edit = _self.state.cards_edit_signal()
            => move {
                let pair_type:PairType = (left_mode, right_mode).into();
                let is_edit = *is_edit;
                let game_mode = *game_mode;
                
                match (game_mode, index) {
                    (Some(game_mode), Some(index)) => {Some(
                        elem!(templates::main_pair(pair_type, is_edit), {
                            .apply_if(is_edit, |dom| {
                                apply_methods!(dom, {
                                    .with_data_id!("delete", {
                                        .event(clone!(state, index => move |evt:events::Click| {
                                            state.pairs.lock_mut().remove(index);
                                        }))
                                    })
                                })
                            })
                            .with_data_id!("left", {
                                .apply(clone!(state, pair_type, game_mode, left_data, right_data, is_edit => move |dom| 
                                    CardDom::render(CardDom::new(pair_type, Side::Left, state, game_mode, left_data, right_data, is_edit), dom)
                                ))
                            })
                            .with_data_id!("right", {
                                .apply(clone!(state, pair_type, game_mode, left_data, right_data, is_edit => move |dom| 
                                    CardDom::render(CardDom::new(pair_type, Side::Right, state, game_mode, left_data, right_data, is_edit), dom)
                                ))
                            })
                            .with_data_id!("number", {
                                .text(&format!("{}", index+1))
                            })
                        })
                    )},
                    _ => None
                }
            }
        }
    }
    fn render(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(Self::dom_signal(_self))
        })
    }
}

struct CardDom {
    pair_type: PairType, 
    side: Side, 
    state: Rc<State>, 
    game_mode: GameMode, 
    left_data: Mutable<Option<String>>, 
    right_data: Mutable<Option<String>>, 
    is_edit: bool,
    is_hover: Mutable<bool>
}

enum Side {
    Left,
    Right
}

impl CardDom {
    fn new( pair_type: PairType, side: Side, state: Rc<State>, game_mode: GameMode, left_data: Mutable<Option<String>>, right_data: Mutable<Option<String>>, is_edit: bool) -> Rc<Self> {
        Rc::new(Self {
            pair_type, 
            side, 
            state, 
            game_mode, 
            left_data, 
            right_data, 
            is_edit,
            is_hover: Mutable::new(false)
        })
    }

    fn flip_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let hover = self.is_hover.signal(),
            let step = self.state.step.signal()
            => {
                let hover = *hover;
                let step = *step;

                if hover && step == Step::Two {
                    true 
                } else {
                    false
                }
            }
        }
    }
    fn render(_self:Rc<Self>, dom: DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement> {


        let pair_type = _self.pair_type;

        let (is_text, is_image) = match _self.side {
            Side::Left => (pair_type.left_is_text(), pair_type.left_is_image()),
            Side::Right => (pair_type.right_is_text(), pair_type.right_is_image()),
        };

        let (this_data, other_data) = match _self.side {
            Side::Left => (_self.left_data.clone(), _self.right_data.clone()),
            Side::Right => (_self.right_data.clone(), _self.left_data.clone()),
        };

        apply_methods!(dom, {
            .class_signal("flip-card-clicked", _self.flip_signal())
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
            .apply_if(is_text, clone!(this_data,other_data, _self => move |dom| {
                apply_text(_self.state.clone(), dom, _self.game_mode, this_data, other_data, _self.is_edit)
            }))
            .apply_if(is_image, clone!(this_data, other_data, _self => move |dom| {
                apply_image(_self.state.clone(), dom, _self.game_mode, this_data, other_data, _self.is_edit)
            }))
        })
    }

}

fn apply_text(state: Rc<State>, dom: DomBuilder<HtmlElement>, game_mode: GameMode, card_data:Mutable<Option<String>>, other_card_data: Mutable<Option<String>>, is_edit: bool) -> DomBuilder<HtmlElement> {
    let text_signal = card_data.signal_cloned().map(|x| x.unwrap_or("".to_string()));

    if is_edit {
        apply_methods!(dom, {
            .with_data_id!("text-contents" => HtmlTextAreaElement, {
                .event_preventable(|evt:events::DragOver| {
                    if let Some(data_transfer) = evt.data_transfer() {
                        if data_transfer.types().index_of(&JsValue::from_str(SEARCH_THUMBNAIL_DATA_TRANSFER), 0) != -1 {
                            evt.prevent_default();
                        }
                    }
                })
                .property_signal("value",text_signal)
                .with_node!(elem => {
                    .event(clone!(card_data, other_card_data, game_mode => move |evt:events::Input| {
                        let value = elem.value();
                        if game_mode == GameMode::Duplicate {
                            other_card_data.set_neq(Some(value.clone()));
                        }
                        card_data.set_neq(Some(value));
                    }))
                })
            })
        })
    } else {
        apply_methods!(dom, {
            .with_data_id!("text-contents", {
                .text_signal(text_signal)
            })
        })
    }
}

fn apply_image(state: Rc<State>, dom: DomBuilder<HtmlElement>, game_mode: GameMode, card_data:Mutable<Option<String>>, other_card_data: Mutable<Option<String>>, is_edit: bool) -> DomBuilder<HtmlElement> {
    if is_edit {
        apply_methods!(dom, {
            .with_data_id!("image", {
                .class_signal("hidden", card_data.signal_ref(|x| x.is_none()))
                .property_signal("src", card_data.signal_ref(|id| {
                    match id {
                        None => "".to_string(),
                        Some(id) => SimpleImage::from((id.as_ref(), MediaLibraryKind::Global)).full_src()
                    }
                })) 
            })
            .with_data_id!("image-waiting", {
                .class_signal("hidden", card_data.signal_ref(|x| x.is_some()))
            })
            .event_preventable(|evt:events::DragOver| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if data_transfer.types().index_of(&JsValue::from_str(SEARCH_THUMBNAIL_DATA_TRANSFER), 0) != -1 {
                        evt.prevent_default();
                    }
                }
            })

            .event(clone!(card_data => move |evt:events::Drop| {
                if let Some(data_transfer) = evt.data_transfer() {
                    if let Some(img_id) = data_transfer.get_data(SEARCH_THUMBNAIL_DATA_TRANSFER).ok() {
                        card_data.set_neq(Some(img_id));
                    }
                }
            }))
        })
    } else {
        apply_methods!(dom, {
            .with_data_id!("image", {
                .class_signal("hidden", card_data.signal_ref(|x| x.is_none()))
                .property_signal("src", card_data.signal_ref(|id| {
                    match id {
                        None => "".to_string(),
                        Some(id) => SimpleImage::from((id.as_ref(), MediaLibraryKind::Global)).full_src()
                    }
                })) 
            })
            .with_data_id!("image-waiting", {
                .class_signal("hidden", card_data.signal_ref(|x| x.is_some()))
            })
        })
    }
}
