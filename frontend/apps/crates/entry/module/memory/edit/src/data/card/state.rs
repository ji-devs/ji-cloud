use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::clone;
use shared::{
    domain::image::ImageId,
    domain::audio::AudioId,
    media::MediaLibrary
};
use crate::data::{raw, history::{self, History}};

#[derive(Debug, Clone)]
pub enum Card {
    Text(Mutable<String>),
    Image(Mutable<Option<(ImageId, MediaLibrary)>>),
    Audio(Mutable<Option<(AudioId, MediaLibrary)>>)
}

impl Card {
    pub fn new_text(data: String) -> Self {
        Self::Text(Mutable::new(data))
    }
    pub fn new_image(data: Option<(ImageId, MediaLibrary)>) -> Self {
        Self::Image(Mutable::new(data))
    }
    pub fn new_audio(data: Option<(AudioId, MediaLibrary)>) -> Self {
        Self::Audio(Mutable::new(data))
    }

    pub fn as_text_mutable(&self) -> &Mutable<String> {
        match self {
            Self::Text(m) => m,
            _ => panic!("not a text type!") 
        }
    }
    pub fn as_image_mutable(&self) -> &Mutable<Option<(ImageId, MediaLibrary)>> {
        match self {
            Self::Image(m) => m,
            _ => panic!("not an image type!") 
        }
    }
    pub fn as_audio_mutable(&self) -> &Mutable<Option<(AudioId, MediaLibrary)>> {
        match self {
            Self::Audio(m) => m,
            _ => panic!("not an audio type!") 
        }
    }

}


impl From<raw::Card> for Card {
    fn from(raw_card:raw::Card) -> Self {
        match raw_card {
            raw::Card::Text(x) => Card::new_text(x),
            raw::Card::Image(x) => Card::new_image(x),
            raw::Card::Audio(x) => Card::new_audio(x),
        }
    }
}

impl From<Card> for raw::Card {
    fn from(card:Card) -> Self {
        match card {
            Card::Text(x) => raw::Card::Text(x.get_cloned()),
            Card::Image(x) => raw::Card::Image(x.get_cloned()),
            Card::Audio(x) => raw::Card::Audio(x.get_cloned()),
        }
    }
}
