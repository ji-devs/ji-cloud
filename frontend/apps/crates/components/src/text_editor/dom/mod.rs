pub(super) mod text_editor_controls;
pub(super) mod wysiwyg;

pub use super::dom::{
    text_editor_controls::text_editor_controls::render as render_controls,
    wysiwyg::render as render_wysiwyg,
};
