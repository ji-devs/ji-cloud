use crate::data::{*, raw::*};

pub const DEBUG_STEP:StepRaw = StepRaw::Two;
pub const DEBUG_THEME_INDEX:usize = 0;

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
