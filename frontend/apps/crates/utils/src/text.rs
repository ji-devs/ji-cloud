use unicode_segmentation::UnicodeSegmentation;

use crate::unwrap::UnwrapJiExt;

pub fn limit_text(max_len: usize, text: String) -> String {
    let len = text.graphemes(true).count();

    if len > max_len {
        let cutoff_grapheme_byte = text.grapheme_indices(true).nth(max_len).unwrap_ji().0;

        text[..cutoff_grapheme_byte].to_string()
    } else {
        text
    }
}
