use crate::base::state::Base;
use std::{rc::Rc};
use std::cell::{RefCell, Cell};
use dominator::{clone, animation::{MutableAnimation}};
use shared::domain::jig::module::body::legacy::activity::{Puzzle as RawPuzzle, PuzzleItem as RawPuzzleItem};
use web_sys::HtmlCanvasElement;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal}
};
use utils::{prelude::*,drag::Drag, image_effects::ImageEffect, resize::{resize_info_signal, ResizeInfo}, math::mat4::Matrix4};
use awsm_web::canvas::get_2d_context;
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::JsCast;
use dominator_helpers::futures::AsyncLoader;

pub struct Puzzle {
    pub base: Rc<Base>,
    pub raw: RawPuzzle,
    pub init_phase: Mutable<InitPhase>,
}

pub struct PuzzlePreview {
    pub game: Rc<PuzzleGame>,
    pub animation: MutableAnimation,
    pub loader: AsyncLoader,
}

pub struct PuzzleGame {
    pub base: Rc<Base>,
    pub raw: RawPuzzle,
    pub effects: ImageEffect,
    pub cutouts_canvas: HtmlCanvasElement,
    pub cutouts_ctx: CanvasRenderingContext2d,
    pub click_canvas: HtmlCanvasElement,
    pub click_ctx: CanvasRenderingContext2d,
    pub items: Vec<Rc<PuzzleItem>>,
    pub drag_index: Cell<Option<usize>> 
}

pub struct PuzzleItem {
    pub base: Rc<Base>,
    pub raw: RawPuzzleItem,
    pub completed: Cell<bool>,
    pub orig_transform_matrix: Matrix4, 
    pub curr_transform_matrix: RefCell<Matrix4>,
    pub drag: RefCell<Option<Rc<Drag>>>,
}

#[derive(Clone)]
pub enum InitPhase {
    Loading,
    Preview(Rc<PuzzlePreview>),
    Playing(Rc<PuzzleGame>)
}

impl Puzzle {
    pub fn new(base: Rc<Base>, raw: RawPuzzle) -> Rc<Self> {
        let _self = Rc::new(Self { 
            base,
            raw,
            init_phase: Mutable::new(InitPhase::Loading),
        });

        _self.base.insert_start_listener(clone!(_self => move || {
            _self.clone().on_start();
        }));

        _self
    }

    pub fn game_signal(&self) -> impl Signal<Item = (InitPhase, ResizeInfo)> {
        map_ref! {
            let init_phase = self.init_phase.signal_cloned(),
            let resize_info = resize_info_signal()
            => {
                (init_phase.clone(), resize_info.clone())
            }
        }
    }
}

impl PuzzlePreview {
    pub fn new(parent: &Puzzle, cutouts_canvas: HtmlCanvasElement, effects: ImageEffect) -> Rc<Self> {
        let game = PuzzleGame::new(parent, cutouts_canvas, effects);
        for item in game.items.iter() {
            *item.curr_transform_matrix.borrow_mut() = Matrix4::identity();
        }

        Rc::new(Self {
            game,
            animation: MutableAnimation::new(crate::config::PUZZLE_PREVIEW_DURATION),
            loader: AsyncLoader::new()
        })

    }
}

impl PuzzleGame {
    pub fn new(parent: &Puzzle, cutouts_canvas: HtmlCanvasElement, effects: ImageEffect) -> Rc<Self> {


        let cutouts_ctx = get_2d_context(&cutouts_canvas, None).unwrap_ji();


        let click_canvas: HtmlCanvasElement = web_sys::window()
            .unwrap_ji()
            .document()
            .unwrap_ji()
            .create_element("canvas")
            .unwrap_ji()
            .unchecked_into();

        let click_ctx = get_2d_context(&click_canvas, None).unwrap_ji();

        let items = parent.raw.items
            .iter()
            .map(|raw| {
                PuzzleItem::new(parent.base.clone(), &effects, raw.clone())
            })
            .collect();

        let _self = Rc::new(Self { 
            base: parent.base.clone(),
            raw: parent.raw.clone(),
            effects,
            cutouts_canvas,
            cutouts_ctx,
            click_canvas,
            click_ctx,
            items,
            drag_index: Cell::new(None)
        });

        _self
    }
}


impl PuzzleItem{
    pub fn new(base: Rc<Base>, _effects: &ImageEffect, raw: RawPuzzleItem) -> Rc<Self> {
        let orig_transform_matrix = match raw.hotspot.transform_matrix {
            None => Matrix4::identity(),
            Some(values) => Matrix4::new_direct(values)
        };

        Rc::new(Self {
            base,
            raw,
            completed: Cell::new(false),
            orig_transform_matrix: orig_transform_matrix.clone(),
            curr_transform_matrix: RefCell::new(orig_transform_matrix),
            drag: RefCell::new(None),
        })
    }
}