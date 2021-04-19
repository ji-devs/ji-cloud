use crate::{
    data::state::Mode,
    steps::sidebar::step_1::widgets::dual_list::dom::ColumnSide
};

pub const STR_DONE:&'static str = "Done";
pub const STR_CLEAR:&'static str = "Clear";
pub const STR_CONTINUE:&'static str = "Continue";

pub const STR_INSTRUCTIONS_LABEL:&'static str = "Written instructions";
pub const STR_INSTRUCTIONS_PLACEHOLDER:&'static str = "Type instructions";

pub mod steps_nav {
    pub const STR_CONTENT:&'static str = "Content";
    pub const STR_DESIGN:&'static str = "Design";
    pub const STR_SETTINGS:&'static str = "Settings";
    pub const STR_PREVIEW:&'static str = "Preview";
}

pub mod error {
    pub const STR_SINGLE_LIST_NUM_WORDS:&'static str = "We recommend at least 2 pairs to create a memory game. Add more words to your list.";
}
pub mod confirm {
    pub const STR_DELETE_PAIR_HEADER:&'static str = "Are you sure you want to delete the pair?";
    pub const STR_DELETE_PAIR_CONFIRM:&'static str = "Delete pair";
    pub const STR_DELETE_PAIR_CANCEL:&'static str = "Keep pair";
}

#[allow(non_snake_case)]
pub const fn STR_HEADER(side: ColumnSide, mode: Mode) -> &'static str {
    match mode {
        Mode::BeginsWith => {
            match side {
                ColumnSide::Left => "Word",
                ColumnSide::Right => "First letter",
            }
        },
        Mode::Riddles => {
            match side {
                ColumnSide::Left => "Riddle",
                ColumnSide::Right => "Answer",
            }
        },
        Mode::Opposites => {
            match side {
                ColumnSide::Left => "Word",
                ColumnSide::Right => "Opposite",
            }
        },
        Mode::Synonymns => {
            match side {
                ColumnSide::Left => "Word",
                ColumnSide::Right => "Synonymn",
            }
        },
        Mode::Translate => {
            match side {
                ColumnSide::Left => "Word",
                ColumnSide::Right => "Translation",
            }
        },
        _ => {
            match side {
                ColumnSide::Left => "LEFT",
                ColumnSide::Right => "RIGHT",
            }
        }
    }
}
