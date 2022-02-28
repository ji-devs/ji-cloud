use std::rc::Rc;

use futures_signals::signal::Mutable;
use utils::themes::ThemeId;

use crate::backgrounds::state::Backgrounds;

pub trait DesignExt {
    fn get_backgrounds(&self) -> Rc<Backgrounds>;

    fn get_theme(&self) -> Mutable<ThemeId>;

    fn set_theme(&self, theme: ThemeId);

    // might make sense to add the following
    // fn get_text_editor() -> Rc<TextEditorState>;
    // fn get_stickers(&self) -> Rc<Stickers<Sticker>>;
}
