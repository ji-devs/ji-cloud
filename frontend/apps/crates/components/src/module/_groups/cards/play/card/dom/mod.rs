pub(super) mod common;
pub mod dynamic;
pub mod empty;
pub mod plain;

pub use common::*;
pub use dynamic::*;
pub use empty::*;
pub use plain::*;

pub const FLIPPED_AUDIO_EFFECT: &str = "module/cards/flip_card_3.mp3";
