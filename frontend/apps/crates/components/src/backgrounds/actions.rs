use futures_signals::{
    map_ref,
    signal_vec::{SignalVecExt, SignalVec, MutableVec},
    signal::{Signal, SignalExt, Mutable, ReadOnlyMutable},
};
use super::state::*; 
use shared::domain::jig::module::body::Background;

pub enum Layer {
    One,
    Two
}

impl Backgrounds {
    pub fn delete_layer(&self, layer:Layer) {
        self.get_layer(layer).set(None);
        self.call_change();
    }

    pub fn set_layer(&self, layer:Layer, bg: Background) {
        self.get_layer(layer).set(Some(bg));
        self.call_change();
    }

    //helper
    fn get_layer(&self, layer:Layer) -> &Mutable<Option<Background>> {
        match layer {
            Layer::One => &self.layer_1,
            Layer::Two => &self.layer_2,
        }
    }

    // Internal - saving/history is done on the module level
    fn call_change(&self) {
        if let Some(on_change) = self.on_change.borrow().as_ref() {
            (on_change) (self.to_raw());
        }
    }
}
