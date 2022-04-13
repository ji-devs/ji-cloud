use crate::base::game::state::{CardPairId, Game};
use components::module::_groups::cards::lookup::Side;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::{
    ThemeId,
    _groups::cards::{Card, Mode},
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::drag::Drag;
use utils::math::BoundsF64;
use web_sys::HtmlElement;

pub type CardTop = CardChoice<TopPhase>;
pub type CardBottom = CardChoice<BottomPhase>;

#[derive(Clone)]
pub struct CardChoice<P: Clone> {
    pub game: Rc<Game>,
    pub phase: Mutable<P>,
    pub card: Card,
    pub other: Card,
    pub pair_id: usize,
    pub side: Side,
    pub theme_id: ThemeId,
    pub mode: Mode,
    pub elem: RefCell<Option<HtmlElement>>,
}

type IsDragOver = bool;
#[derive(Clone)]
pub enum TopPhase {
    Empty(Mutable<IsDragOver>),
    Landed,
}

#[derive(Clone)]
pub enum BottomPhase {
    Show,
    Remove,
}

pub struct CardDrag {
    pub game: Rc<Game>,
    pub drag: Drag,
    pub elem: RefCell<Option<HtmlElement>>,
    pub is_over: Mutable<Option<usize>>,
    pub card: Card,
    pub other: Card,
    pub pair_id: usize,
    pub side: Side,
    pub theme_id: ThemeId,
    pub mode: Mode,
}

impl CardChoice<TopPhase> {
    pub fn new(game: Rc<Game>, pair: CardPairId) -> Self {
        let theme_id = game.base.theme_id;
        let mode = game.base.mode;
        let swap = game.base.settings.swap;

        let CardPairId(card, other, pair_id) = pair;

        let side = if !swap { Side::Left } else { Side::Right };

        let (card, other) = if !swap { (card, other) } else { (other, card) };

        Self {
            game,
            phase: Mutable::new(TopPhase::Empty(Mutable::new(false))),
            card,
            other,
            pair_id,
            side,
            theme_id,
            mode,
            elem: RefCell::new(None),
        }
    }

    pub fn set_drag_over(&self, is_drag_over: bool) {
        match self.phase.get_cloned() {
            TopPhase::Empty(drag_over) => {
                drag_over.set_neq(is_drag_over);
            }
            _ => {}
        }
    }

    pub fn is_drag_over(&self) -> bool {
        match self.phase.get_cloned() {
            TopPhase::Empty(drag_over) => drag_over.get(),
            _ => false,
        }
    }
    pub fn is_landed(&self) -> bool {
        matches!(self.phase.get_cloned(), TopPhase::Landed)
    }
}

impl CardChoice<BottomPhase> {
    pub fn new(game: Rc<Game>, pair: CardPairId) -> Self {
        let theme_id = game.base.theme_id;
        let mode = game.base.mode;
        let swap = game.base.settings.swap;

        let CardPairId(card, other, pair_id) = pair;

        let side = if !swap { Side::Right } else { Side::Left };

        let (card, other) = if !swap { (other, card) } else { (card, other) };

        CardChoice {
            game,
            phase: Mutable::new(BottomPhase::Show),
            card,
            other,
            pair_id,
            side,
            theme_id,
            mode,
            elem: RefCell::new(None),
        }
    }
}

impl CardDrag {
    pub fn new<S: Clone>(choice: CardChoice<S>, elem: HtmlElement, x: i32, y: i32) -> Self {
        let CardChoice {
            game,
            phase: _,
            card,
            other,
            pair_id,
            side,
            theme_id,
            mode,
            ..
        } = choice;

        let drag = Drag::new_anchor_element_resize(x, y, &elem, true);

        CardDrag {
            is_over: Mutable::new(None),
            drag,
            game,
            card,
            other,
            pair_id,
            side,
            theme_id,
            mode,
            elem: RefCell::new(None),
        }
    }

    pub fn get_bounds(&self) -> Option<BoundsF64> {
        self.elem
            .borrow()
            .as_ref()
            .map(|elem| elem.get_bounding_client_rect())
            .map(|rect| rect.into())
    }
}
