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
    pub fn new(raw:Option<&RawBackgrounds>, on_change: Option<Box<dyn Fn(RawBackgrounds)>>) -> Self {
   
        let (layer_1, layer_2) = {
            if let Some(raw) = raw {
                (
                    Mutable::new(raw.layer_1.clone()),
                    Mutable::new(raw.layer_2.clone())
                )
            } else {
                (Mutable::new(None), Mutable::new(None))
            }
        };

        Self {
            layer_1,
            layer_2,
            on_change: RefCell::new(on_change),
        }
    }

    pub fn to_raw(&self) -> RawBackgrounds {
        RawBackgrounds {
            layer_1: self.layer_1.get_cloned(),
            layer_2: self.layer_1.get_cloned(),
        }
    }
}

