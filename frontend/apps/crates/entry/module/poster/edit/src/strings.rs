#![allow(dead_code)] // this should be remove eventually

pub const STR_DONE: &str = "Done";
pub const STR_CLEAR: &str = "Clear";
pub const STR_CONTINUE: &str = "Continue";

pub mod steps_nav {
    pub const STR_THEMES: &str = "Themes";
    pub const STR_BACKGROUND: &str = "Background";
    pub const STR_CONTENT: &str = "Content";
    pub const STR_PREVIEW: &str = "Preview";
}

pub mod mode {
    pub const STR_TITLE: &str = "Create a Poster";
    pub const STR_PRINTABLES_LABEL: &str = "Printables";
    pub const STR_TALKING_PICTURES_LABEL: &str = "Talking Pictures";
    pub const STR_COMICS_LABEL: &str = "Comics";
    pub const STR_TIMELINE_LABEL: &str = "Timeline";
    pub const STR_FAMILY_TREE_LABEL: &str = "Family Tree";
    pub const STR_POSTER_LABEL: &str = "Poster";
}

pub mod step_3 {
    pub const STR_LABEL: &str = "Text";
    pub const STR_PLACEHOLDER: &str = "Type the text";
}

pub mod step_4 {
    pub const STR_HEADER_HINT: &str = "See the selected areas as highlighted hints.";
    pub const STR_HEADER_NEXT: &str = "Continue to next activity";
    pub const STR_HINT_NONE: &str = "No highlights";
    pub const STR_HINT_HIGHLIGHT: &str = "Highlight clickable areas";
    pub const STR_NEXT_CONTINUE: &str = "By clicking on continue";
    pub const STR_NEXT_SELECT_ALL: &str = "By clicking all items";
    pub const STR_NEXT_SELECT_SOME_PREFIX: &str = "By clicking a minimum of";
    pub const STR_NEXT_SELECT_SOME_SUFFIX: &str = "items";
}
