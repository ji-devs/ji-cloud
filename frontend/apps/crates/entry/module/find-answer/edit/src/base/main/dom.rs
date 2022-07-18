use super::state::*;
use crate::base::state::Phase;

use components::{
    backgrounds::dom::render_backgrounds,
    module::_common::edit::prelude::*,
    stickers::{
        dom::{render_stickers, render_stickers_raw},
        state::Sticker,
    },
    traces::edit::TracesEdit,
};
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::find_answer::Step;
use std::rc::Rc;

impl DomRenderable for Main {
    fn render(state: Rc<Main>) -> Dom {
        html!("empty-fragment", {
            .future(state.base.stickers.selected_index.signal_ref(|selected| selected.clone()).for_each(clone!(state => move |selected| {
                if state.base.step.get_cloned() == Step::Three {
                    let is_empty = state.base.questions.lock_ref().is_empty();
                    if is_empty {
                        if let Some(selected) = selected {
                            if let Some(Sticker::Text(text)) = state.base.stickers.list.lock_ref().get(selected) {
                                let text = text.editor.get_text_value();
                                state.base.add_question(text, Some(selected));
                                state.base.current_question.set(Some(0));
                            }
                        }
                    }
                }

                async {}
            })))
            .child(html!("img-ui", {
                .property("path", "jig/play/design-grid.svg")
                .style("height", "100%")
            }))
            .children_signal_vec(
                state.base.phase.signal_cloned().map(clone!(state => move |phase| {
                    match phase {
                        Phase::Layout => {
                            vec![
                                render_stickers(state.base.stickers.clone())
                            ]
                        },
                        Phase::Trace(traces) => {
                            let raw_stickers = state.base.stickers.to_raw();
                            let theme_id = state.base.theme_id.get();

                            vec![
                                render_stickers_raw(&raw_stickers, theme_id),
                                TracesEdit::render(traces.clone()),
                            ]
                        }
                    }
                }))
                .to_signal_vec()
            )
        })
    }
}

impl MainDomRenderable for Main {
    fn render_bg(state: Rc<Main>) -> Option<Dom> {
        Some(render_backgrounds(state.base.backgrounds.clone(), None))
    }
}
