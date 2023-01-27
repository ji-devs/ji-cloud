use super::state::*;
use crate::base::state::Phase;

use components::{
    backgrounds::dom::render_backgrounds,
    module::_common::edit::prelude::*,
    stickers::{dom::render_stickers, state::Sticker},
    traces::edit::TracesEdit,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::find_answer::{QuestionField, Step};
use std::rc::Rc;
use utils::unwrap::UnwrapJiExt;

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .future(state.base.question_field.signal_cloned().for_each(clone!(state => move |question| {
                if let QuestionField::Text(index) = question {
                    let text = state.base.stickers.get_as_text(index).unwrap_ji();
                    text.can_delete.set_neq(false);
                    text.highlight.set_neq(true);
                }
                async {}
            })))
            .future(state.base.stickers.selected_index.signal_ref(|selected| selected.clone()).for_each(clone!(state => move |selected| {
                if state.base.step.get_cloned() == Step::Three {
                    let is_empty = state.base.questions.lock_ref().is_empty();
                    if is_empty {
                        if let Some(selected) = selected {
                            if let Some(Sticker::Text(text)) = state.base.stickers.list.lock_ref().get(selected) {
                                let text = text.get_text_value();
                                state.base.add_question(text, Some(selected));
                                state.base.current_question.set(Some(0));
                            }
                        }
                    }
                }
                async {}
            })))
            .child(html!("img-ui", {
                .prop("path", "jig/play/design-grid-jig.svg")
                .style("height", "100%")
                .style("width", "100%")
                .style("display", "block")
            }))
            // Normally we would render raw stickers for `Trace`, but because we need to have access to the renderer_ref on a sticker to
            // show the question's text, we use regular stickers. The traces component will overlay the stickers so that they cannot be edited.
            .child(render_stickers(state.base.stickers.clone()))
            .child_signal(
                state.base.phase.signal_cloned().map(move |phase| {
                    match phase {
                        Phase::Trace(traces) => {
                            Some(TracesEdit::render(traces.clone()))
                        }
                        Phase::Layout => None,
                    }
                })
            )
        })
    }
}

impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
