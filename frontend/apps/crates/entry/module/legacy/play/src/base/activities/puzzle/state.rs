use crate::base::state::Base;
use std::rc::Rc;
use std::cell::RefCell;
use dominator::clone;
use shared::domain::jig::module::body::legacy::activity::{Puzzle as RawPuzzle, PuzzleItem as RawPuzzleItem};
use web_sys::HtmlCanvasElement;
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt}
};
use utils::{prelude::*, image_effects::ImageEffect, resize::{resize_info_signal, ResizeInfo}};
use awsm_web::canvas::get_2d_context;
use web_sys::CanvasRenderingContext2d;

pub struct Puzzle {
    pub base: Rc<Base>,
    pub raw: RawPuzzle,
    pub init_phase: Mutable<InitPhase>,
}

pub struct PuzzleGame {
    pub base: Rc<Base>,
    pub raw: RawPuzzle,
    pub effects: ImageEffect,
    pub cutouts_canvas: HtmlCanvasElement,
    pub cutouts_ctx: CanvasRenderingContext2d,
    pub items: Vec<Rc<PuzzleItem>> 
}

pub struct PuzzleItem {
    pub raw: RawPuzzleItem
}

#[derive(Clone)]
pub enum InitPhase {
    Loading,
    Playing(Rc<PuzzleGame>)
}

impl Puzzle {
    pub fn new(base: Rc<Base>, raw: RawPuzzle) -> Rc<Self> {
        let _self = Rc::new(Self { 
            base,
            raw,
            init_phase: Mutable::new(InitPhase::Loading)
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

impl PuzzleGame {
    pub fn new(parent: &Puzzle, cutouts_canvas: HtmlCanvasElement, effects: ImageEffect) -> Rc<Self> {


        let cutouts_ctx = get_2d_context(&cutouts_canvas, None).unwrap_ji();

        let items = parent.raw.items
            .iter()
            .map(|raw| {
                PuzzleItem::new(&effects, raw.clone())
            })
            .collect();

        let _self = Rc::new(Self { 
            base: parent.base.clone(),
            raw: parent.raw.clone(),
            effects,
            cutouts_canvas,
            cutouts_ctx,
            items
        });

        _self
    }
}


impl PuzzleItem{
    pub fn new(effects: &ImageEffect, raw: RawPuzzleItem) -> Rc<Self> {
        Rc::new(Self {
            raw
        })
    }
}