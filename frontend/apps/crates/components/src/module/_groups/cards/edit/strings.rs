use crate::lists::dual::dom::ColumnSide;
use shared::domain::jig::module::body::_groups::cards::Mode;

pub const STR_DONE: &str = "Done";
pub const STR_CLEAR: &str = "Clear";
pub const STR_CONTINUE: &str = "Continue";

pub const STR_CREATE_NEW_LIST: &str = "Create new list";

pub const STR_HEADER_ADD_PAIR: &str = "Add pair";

pub mod steps_nav {
    pub const STR_CONTENT: &str = "Content";
    pub const STR_DESIGN: &str = "Design";
    pub const STR_SETTINGS: &str = "Settings";
    pub const STR_PREVIEW: &str = "Preview";
}

pub mod error {
    pub const STR_SINGLE_LIST_NUM_WORDS: &str =
        "We recommend at least 2 pairs to create a memory game. Add more words to your list.";
}
pub mod confirm {
    pub const STR_DELETE_PAIR_HEADER: &str = "Are you sure you want to delete the pair?";
    pub const STR_DELETE_PAIR_CONFIRM: &str = "Delete pair";
    pub const STR_DELETE_PAIR_CANCEL: &str = "Keep pair";
}

#[allow(non_snake_case)]
pub const fn STR_HEADER(side: ColumnSide, mode: Mode) -> &'static str {
    match mode {
        Mode::BeginsWith => match side {
            ColumnSide::Left => "Word",
            ColumnSide::Right => "First letter",
        },
        Mode::Riddles => match side {
            ColumnSide::Left => "Riddle",
            ColumnSide::Right => "Answer",
        },
        Mode::Opposites => match side {
            ColumnSide::Left => "Word",
            ColumnSide::Right => "Opposite",
        },
        Mode::Synonyms => match side {
            ColumnSide::Left => "Word",
            ColumnSide::Right => "Synonym",
        },
        Mode::Translate => match side {
            ColumnSide::Left => "Word",
            ColumnSide::Right => "Translation",
        },
        _ => match side {
            ColumnSide::Left => "LEFT",
            ColumnSide::Right => "RIGHT",
        },
    }
}
