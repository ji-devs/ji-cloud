use dominator::{html, clone, Dom};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use super::state::*;

pub fn render_post_preview(state:Rc<PostPreview>) -> Dom {
    //TODO!
    html!("post-preview", {
    })
}

/*
<post-preview ${argsToAttrs(props)}>
            <post-preview-action slot="action-1of3" kind="1of3"></post-preview-action>
            <post-preview-action slot="action-matching" kind="matching"></post-preview-action>
            <post-preview-action slot="action-flashcards" kind="flashcards"></post-preview-action>
            <post-preview-action slot="action-print" kind="print"></post-preview-action>
            <post-preview-action slot="action-continue" kind="continue"></post-preview-action>
        </post-preview>
        */
