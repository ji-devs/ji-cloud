use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use shared::{domain::{image::ImageId, jig::module::body::{Text as RawText, Transform}}, media::MediaLibrary};

use components::transform::{
    state::TransformState,
};

#[derive(Clone)]
pub struct Text {
    pub value: Mutable<String>,
    pub transform: Rc<TransformState>,
}

impl Text {
    pub fn new(text:&RawText) -> Self {
        let text = text.clone();
        Self {
            value: Mutable::new(text.value),
            transform: Rc::new(TransformState::new(text.transform, None)),
        }
    }

}

