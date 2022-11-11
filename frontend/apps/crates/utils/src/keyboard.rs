use dominator::events::{KeyDown, KeyUp};

use crate::{resize::get_resize_info, unwrap::UnwrapJiExt};

pub const MOVE_MULTIPLIER: f64 = 10.0;
pub const MOVE_AMOUNT_PX: f64 = 1.0;

/// Map of keyboard keys used to perform various actions in the system.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Key {
    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown,
    Delete,
    Other(String),
}

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub is_osx: bool,
    pub shift: bool,
    pub alt: bool,
    pub ctrl_cmd: bool,
    pub key: Key,
}

impl Key {
    pub fn is_move_key(&self) -> bool {
        match self {
            Self::ArrowLeft | Self::ArrowRight | Self::ArrowUp | Self::ArrowDown => true,
            _ => false,
        }
    }
    pub fn translation_from_key(&self) -> (f64, f64) {
        let resize_info = get_resize_info();
        match self {
            Self::ArrowLeft => resize_info.get_px_normalized(-MOVE_AMOUNT_PX, 0.0),
            Self::ArrowRight => resize_info.get_px_normalized(MOVE_AMOUNT_PX, 0.0),
            Self::ArrowUp => resize_info.get_px_normalized(0.0, -MOVE_AMOUNT_PX),
            Self::ArrowDown => resize_info.get_px_normalized(0.0, MOVE_AMOUNT_PX),
            _ => (0.0, 0.0),
        }
    }
}

impl From<String> for Key {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ArrowLeft" => Self::ArrowLeft,
            "ArrowUp" => Self::ArrowUp,
            "ArrowRight" => Self::ArrowRight,
            "ArrowDown" => Self::ArrowDown,
            "Delete" => Self::Delete,
            other => Self::Other(other.into()),
        }
    }
}

/// Helper to implement `KeyUp` and `KeyDown` events.
macro_rules! impl_from_key_event {
    ($event_type:ident) => {
        impl From<$event_type> for KeyEvent {
            fn from(value: $event_type) -> Self {
                let user_agent: String = web_sys::window()
                    .unwrap_ji()
                    .navigator()
                    .user_agent()
                    .unwrap_ji();

                Self {
                    is_osx: user_agent.contains("Mac OS X"),
                    shift: value.shift_key(),
                    alt: value.alt_key(),
                    ctrl_cmd: value.ctrl_key(),
                    key: Key::from(value.key()),
                }
            }
        }
    };
}

impl_from_key_event!(KeyDown);
impl_from_key_event!(KeyUp);
