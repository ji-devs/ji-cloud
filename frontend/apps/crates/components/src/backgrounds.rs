pub mod dom;
pub mod actions;

use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};

use std::rc::Rc;
use std::cell::RefCell;
use shared::domain::jig::module::body::{Background, Backgrounds as RawBackgrounds};

pub struct Backgrounds 
{
    pub layer_1: Mutable<Option<Background>>,
    pub layer_2: Mutable<Option<Background>>,
    pub on_change: RefCell<Option<Box<dyn Fn(RawBackgrounds)>>>,
}
impl Backgrounds {
    pub fn new(raw:&RawBackgrounds, on_change: Option<Box<dyn Fn(RawBackgrounds)>>) -> Self {
   
        let layer_1 = Mutable::new(raw.layer_1.clone());
        let layer_2 = Mutable::new(raw.layer_2.clone());

        Self {
            layer_1,
            layer_2,
            on_change: RefCell::new(on_change),
        }
    }
}

