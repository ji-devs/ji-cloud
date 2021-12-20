use components::input::simple_select::SimpleSelectItem;

pub(super) mod actions;
pub mod dom;
pub(super) mod state;

/// Wrapper for [`utils::languages::Language`] so that we can implement SimpleSelect's
/// [`SimpleSelectItem`] trait.
#[derive(Clone)]
pub struct Language(utils::languages::Language);

impl SimpleSelectItem for Language {
    fn value(&self) -> &str {
        self.0.0
    }

    fn label(&self) -> &str {
        self.0.1
    }
}

impl From<utils::languages::Language> for Language {
    fn from(language: utils::languages::Language) -> Self {
        Language(language)
    }
}
