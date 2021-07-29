use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::{signal::{Always, SignalExt, always}, signal_vec::SignalVecExt};
use utils::prelude::*;
use crate::base::{
    state::*,
    card::state::*
};
use shared::domain::jig::module::body::ModeExt;
use components::module::_groups::cards::play::card::dom::{render_dynamic_card_mixin, DynamicCardOptions, Size, NoTransform};


pub fn render(state: Rc<Base>) -> Dom {

    html!("play-main", {
        .property("nCards", state.cards.len() as f64)
        .children(
            //Always render the cards so they take the grid spots
            //"hiding" is via `visiblity`, not `display`
            state.cards
                .iter()
                .map(clone!(state => move |card| {
                    render_main_card(state.clone(), card.clone())
                }))
        )
    })
}

fn render_main_card(state: Rc<Base>, card_state: Rc<CardState>) -> Dom {
    let card_id = &card_state.id;
    let card = &card_state.card;
   
    let theme_id = state.theme_id;
    let mode = state.mode;
    let side = card_state.side;
    let size = Size::Memory;

    let flipped_signal = card_state.is_flipped(&state);
    let transparent_signal = card_state.is_found();
    let hidden_signal = always(false);
    let get_simple_transform = None::<NoTransform>;

    let mut options = DynamicCardOptions::new(
        card,
        theme_id,
        mode,
        side,
        size,
        flipped_signal,
        transparent_signal,
        hidden_signal,
        get_simple_transform,
    );

    render_dynamic_card_mixin(options, |dom| {
        dom
            .event(clone!(state, card_id => move |evt:events::Click| {
                if let Some((id_1, id_2)) = super::actions::card_click(state.clone(), card_id) {
                    super::actions::evaluate(state.clone(), id_1, id_2);
                }
            }))
            .after_inserted(clone!(card_state => move |elem| {
                *card_state.main_elem.borrow_mut() = Some(elem);
            }))
    })
}


/*
 *
pub struct DynamicCardOptions <'a, F, T, H, S, SOut> 
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,
{
    pub card: &'a Card,
    pub back_card: Option<&'a Card>,
    pub flip_on_hover: bool,
    pub flipped: F,
    pub transparent: T,
    pub hidden: H,
    pub get_simple_transform: Option<S>,
    pub theme_id: ThemeId,
    pub size: Size,
    pub mode: Mode,
    //should be set to match card and back_card will automatically
    //use the opposite
    pub side: Side, 
}
impl <'a, F, T, H, S, SOut> DynamicCardOptions <'a, F, T, H, S, SOut> 
where
    F: Signal<Item = bool> + 'static,
    T: Signal<Item = bool> + 'static,
    H: Signal<Item = bool> + 'static,
    S: Fn() -> SOut + 'static,
    SOut: Signal<Item = Option<SimpleTransform>> + 'static,

{
    pub fn new(
        card:&'a Card, 
        theme_id: 
        ThemeId, 
        mode: Mode, 
        side: Side, 
        size: Size,
        flipped: F,
        transparent: T,
        hidden: H,
        get_simple_transform: Option<S>
    ) -> Self {
        Self {
            card,
            theme_id,
            mode,
            side,
            size,
            flipped,
            transparent,
            hidden,
            get_simple_transform,
            //mimic default derive
            back_card: None,
            flip_on_hover: false,

        }
    }
}
*/






/*
pub struct CardOptions <'a> {
    pub card: &'a Card,
    pub back_card: Option<&'a Card>,
    pub flip_on_hover: bool,
    pub flipped: bool,
    pub theme_id: ThemeId,
    pub size: Size,
    pub mode: Mode,
    //should be set to match card and back_card will automatically
    //use the opposite
    pub side: Side, 
}
*/
/*
fn render_card(state: Rc<Base>, card: Rc<CardState>) -> Dom {
    let card_id = &card.id;

    html!("play-card", {
        .property_signal("flipped", card.is_flipped(&state))
        .property("theme", state.theme_id.as_str_id())
        .property("mode", state.mode.as_str_id())
        .property("size", "regular")
        .property("side", card.side.as_str_id())
        .style_signal("visibility", card.is_found().map(|flag| {
            if flag {
                "hidden"
            } else {
                "visible"
            }
        }))
        .child(render_card_media(&card, state.mode, state.theme_id))
        .event(clone!(state, card_id => move |evt:events::Click| {
            if let Some((id_1, id_2)) = super::actions::card_click(state.clone(), card_id) {
                super::actions::evaluate(state.clone(), id_1, id_2);
            }
        }))
        .after_inserted(clone!(card => move |elem| {
            *card.main_elem.borrow_mut() = Some(elem);
        }))
        
        
    })
}
*/
