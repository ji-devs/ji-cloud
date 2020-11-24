use crate::data::*;

pub const THEME_EXAMPLE_TEXT_1:&'static str = "שמש";
pub const THEME_EXAMPLE_TEXT_2:&'static str = "sun";

pub const INITIAL_CARD_TEXTS:&[&'static str] = &[
    "שמש",
    "ירח",
    "כוכב",
    "Sun",
    "Moon",
    "Star",
];

pub const THEME_OPTIONS: &[Theme] = &[
    Theme {
        id: "basic",
        label: "Basic"
    },
    Theme {
        id: "foo",
        label: "Foo"
    },
    Theme {
        id: "bar",
        label: "Bar"
    }
];

pub trait BaseGameStateExt {
    fn base_default() -> raw::BaseGameState {
        raw::BaseGameState {
            pairs: Vec::new(),
            theme_id: "basic".to_string()
        }
    }
    fn default_duplicate() -> raw::BaseGameState {
        raw::BaseGameState {
            pairs: INITIAL_CARD_TEXTS
                .iter()
                .map(|text| {
                    (
                        raw::Card::Text(text.to_string()),
                        raw::Card::Text(text.to_string())
                    )
                })
                .collect(),
            ..Self::base_default()
        }
    }
    fn default_words_and_images() -> raw::BaseGameState {
        raw::BaseGameState {
            pairs: INITIAL_CARD_TEXTS
                .iter()
                .map(|text| {
                    (
                        raw::Card::Text(text.to_string()),
                        raw::Card::Image(None)
                    )
                })
                .collect(),
            ..Self::base_default()
        }
    }
}

impl BaseGameStateExt for raw::BaseGameState {}
