use std::rc::Rc;

use futures_signals::signal::Mutable;
use utils::themes::ThemeId;

use crate::{backgrounds::state::Backgrounds, image::tag::ImageTag};

pub trait DesignExt<Mode> {
    fn get_backgrounds(&self) -> Rc<Backgrounds>;

    fn get_theme(&self) -> Mutable<ThemeId>;

    fn set_theme(&self, theme: ThemeId);

    fn get_image_tag_priorities(&self) -> Option<Vec<ImageTag>>;

    // might make sense to add the following
    // fn get_text_editor() -> Rc<TextEditorState>;
    // fn get_stickers(&self) -> Rc<Stickers<Sticker>>;
}
