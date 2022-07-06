use crate::base::{
    actions::Direction,
    state::{Phase, Question},
};

use super::state::*;
use std::rc::Rc;

use components::{
    audio::input::{AudioInput, AudioInputCallbacks, AudioInputOptions},
    module::_common::prelude::Audio,
    overlay::handle::OverlayHandle,
    tabs::{MenuTab, MenuTabKind},
};
use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use js_sys::Reflect;
use shared::domain::module::body::_groups::design::TraceKind;
use utils::prelude::*;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

fn empty_signal(state: Rc<Step3>) -> impl Signal<Item = bool> {
    state
        .sidebar
        .base
        .questions
        .signal_vec_cloned()
        .len()
        .map(|len| len == 0)
}

fn current_question_idx_signal(
    state: Rc<Step3>,
    question_index: impl Signal<Item = Option<usize>>,
) -> impl Signal<Item = bool> {
    map_ref! {
        let current_question = state.sidebar.base.current_question.signal(),
        let question_index = question_index
            => {
            match (current_question, question_index) {
                (Some(current_index), Some(question_index)) => current_index == question_index,
                _ => false,
            }
        }
    }
}

fn question_default_title_signal(
    question: Rc<Question>,
    index: impl Signal<Item = Option<usize>>,
) -> impl Signal<Item = String> {
    map_ref! {
        let title = question.title.signal_cloned(),
        let index = index
            => {
                let title = title.clone().unwrap_or_default();
                if title.is_empty() {
                    format!("Question {}", index.unwrap_ji() + 1)
                } else {
                    title
                }
            }
    }
}

fn current_question_signal(state: Rc<Step3>) -> impl Signal<Item = Option<(usize, Rc<Question>)>> {
    state
        .sidebar
        .base
        .current_question
        .signal_ref(clone!(state => move |idx| {
            idx.map(clone!(state => move |idx| {
                let question = state.sidebar.base.questions.lock_ref().get(idx).unwrap_ji().clone();
                (idx, question)
            }))
        }))
}

pub fn render(state: Rc<Step3>) -> Dom {
    html!("module-sidebar-body", {
        .property("slot", "body")
        .property_signal("dark", empty_signal(state.clone()).map(|is_empty| !is_empty))
        .child_signal(empty_signal(state.clone()).map(clone!(state => move |is_empty| {
            if is_empty {
                Some(html!("sidebar-empty", {
                    .property("imagePath", "module/_common/edit/sidebar/illustration-trace-area.svg")
                    .child(html!("div", {
                        .child(html!("p", {
                            .text("Select the question if it's already on your page")
                        }))
                        .child(html!("p", {
                            .text("or")
                        }))
                        .child(html!("button-main", {
                            .property("size", "medium")
                            .property("color", "blue")
                            .property("kind", "text")
                            .text("add a question")
                            .event(clone!(state => move|_: events::Click| {
                                state.sidebar.base.add_default_question();
                                state.sidebar.base.current_question.set(Some(0))
                            }))
                        }))
                    }))
                }))
            } else {
                Some(html!("question-container", {
                    .children_signal_vec(
                        state.sidebar.base.questions.signal_vec_cloned().enumerate().map(clone!(state => move |(index, question)| {
                            render_question(state.clone(), index, question.clone())
                        }))
                    )
                }))
            }
        })))
        .child(html!("button-icon-label", {
            .property("slot", "action")
            .property("icon", "circle-+-blue")
            .property("label", "Add a question")
            .property("labelcolor", "blue")
            .event(clone!(state => move|_: events::Click| {
                state.sidebar.base.add_default_question();
                let index = state.sidebar.base.questions.lock_ref().len() - 1;
                state.sidebar.base.current_question.set(Some(index));
            }))
        }))
    })
}

fn close_kebab_menu(elem: &HtmlElement) {
    let _ = Reflect::set(elem, &"visible".into(), &false.into());
}

pub fn render_question(
    state: Rc<Step3>,
    index: ReadOnlyMutable<Option<usize>>,
    question: Rc<Question>,
) -> Dom {
    html!("question-item", {
        .child(html!("fa-button", {
            .property("slot", "toggle")
            .property_signal("icon", current_question_idx_signal(state.clone(), index.signal()).map(|is_current| {
                if is_current {
                    "fa-thin fa-angle-down"
                } else {
                    "fa-thin fa-angle-right"
                }
            }))
            .event(clone!(state, index => move|_: events::Click| {
                let question_index = index.get_cloned().unwrap_ji();
                state.sidebar.base.set_current_question(question_index);
            }))
        }))
        .child_signal(question.is_editing_title.signal_cloned().dedupe().map(clone!(state, index, question => move |is_editing_title| {
            if is_editing_title {
                Some(html!("input-wrapper", {
                    .property("slot", "title")
                    .property("pad", false)
                    .child(html!("input" => HtmlInputElement, {
                        .property_signal("value", question.title.signal_cloned().map(|title| {
                            match title {
                                Some(title) => title,
                                None => "".to_string(),
                            }
                        }))
                        .event(clone!(state, index, question => move |evt: events::Change| {
                            let target = evt.dyn_target::<HtmlInputElement>().unwrap_ji();
                            let value = target.value();
                            let value = value.trim();

                            question.title.set(Some(value.into()));
                            state.sidebar.base.save_question(index.get().unwrap_ji(), question.clone());
                        }))
                        .event(clone!(question => move |_evt: events::Blur| {
                            question.is_editing_title.set_neq(false);
                        }))
                        .after_inserted(|elem| {
                            wasm_bindgen_futures::spawn_local(clone!(elem => async move {
                                gloo_timers::future::TimeoutFuture::new(0).await;
                                let _ = elem.focus();
                            }));
                        })
                    }))
                }))
            } else {
                Some(html!("span", {
                    .style("cursor", "pointer")
                    .property("slot", "title")
                    .text_signal(question_default_title_signal(question.clone(), index.signal_cloned()))
                    .event(clone!(state, index => move|_: events::Click| {
                        let question_index = index.get_cloned().unwrap_ji();
                        state.sidebar.base.set_current_question(question_index);
                    }))
                }))
            }
        })))
        .child_signal(question.is_editing_title.signal_cloned().dedupe().map(clone!(question => move |is_editing| {
            if !is_editing {
                Some(html!("img-ui", {
                    .property("slot", "edit-btn")
                    .style("cursor", "pointer")
                    .property("path", "core/inputs/pencil-blue-darker.svg")
                    .event(clone!(question => move|_: events::Click| {
                        question.is_editing_title.set_neq(!question.is_editing_title.get());
                    }))
                }))
            } else {
                None
            }
        })))
        .child(html!("menu-kebab", {
            .with_node!(kebab_elem => {
                .property("slot", "menu")
                .child(html!("menu-line", {
                    .property("icon", "edit")
                    .property("customLabel", "Rename question")
                    .event(clone!(question, kebab_elem => move |_: events::Click| {
                        question.is_editing_title.set_neq(true);
                        close_kebab_menu(&kebab_elem);
                    }))
                }))
                .child_signal(index.signal_ref(clone!(state, index, kebab_elem => move |current_index| {
                    match current_index {
                        Some(current_index) if *current_index > 0 => {
                            Some(html!("menu-line", {
                                .property("icon", "move-up")
                                .event(clone!(state, index, kebab_elem => move |_: events::Click| {
                                    state.sidebar.base.move_question(index.get().unwrap_ji(), Direction::Up);
                                    close_kebab_menu(&kebab_elem);
                                }))
                            }))
                        }
                        _ => None
                    }
                })))
                .child_signal(index.signal_ref(clone!(state, index, kebab_elem => move |current_index| {
                    match current_index {
                        Some(current_index) if *current_index < state.sidebar.base.questions.lock_ref().len() - 1 => {
                            Some(html!("menu-line", {
                                .property("icon", "move-down")
                                .event(clone!(state, index, kebab_elem => move |_: events::Click| {
                                    state.sidebar.base.move_question(index.get().unwrap_ji(), Direction::Down);
                                    close_kebab_menu(&kebab_elem);
                                }))
                            }))
                        }
                        _ => None
                    }
                })))
                .child(html!("menu-line", {
                    .property("icon", "delete")
                    .property("customLabel", "Delete question")
                    .event(clone!(question, kebab_elem => move |_: events::Click| {
                        question.confirm_delete.set_neq(true);
                        close_kebab_menu(&kebab_elem);
                    }))
                }))
            })
        }))
        .child_signal(current_question_idx_signal(state.clone(), index.signal()).map(clone!(state => move |is_current| {
            if is_current {
                Some(html!("empty-fragment", {
                    .after_removed(clone!(state => move |_| {
                        // Make sure that we're always in the initial phase when rendering a question body
                        state.sidebar.base.phase.set(Phase::Layout);
                        // Make sure that the Question tab is always the first tab set
                        state.sidebar.tab_kind.set(Some(MenuTabKind::Question));
                    }))
                    .style("display", "contents")
                    .child_signal(
                        state.sidebar.tab_kind.signal_cloned().map(clone!(state => move |kind| {
                            match kind {
                                Some(_) => {
                                    Some(html!("menu-tabs", {
                                        .future(state.sidebar.tab_kind.signal_cloned().for_each(clone!(state => move |kind| {
                                            state.sidebar.base.phase.set(
                                                match kind {
                                                    Some(MenuTabKind::Answer) => Phase::new_trace_unchecked(state.sidebar.base.clone(), TraceKind::Correct),
                                                    _ => Phase::Layout,
                                                }
                                            );

                                            async {}
                                        })))
                                        .style("margin-top", "12px")
                                        .children(&mut [
                                            render_tab(MenuTabKind::Question, state.sidebar.tab_kind.clone()),
                                            render_tab(MenuTabKind::Answer, state.sidebar.tab_kind.clone()),
                                            html!("module-sidebar-body", {
                                                .property("slot", "body")
                                                .child_signal(
                                                    //based on the selected tab kind, create and render the tab state
                                                    state.sidebar.tab_kind.signal_cloned()
                                                        .map(clone!(state => move |tab| {
                                                            tab.map(|tab| {
                                                                render_tab_body(state.clone(), tab.into())
                                                            })
                                                        }))
                                                )
                                            })
                                        ])
                                    }))
                                }
                                None => {
                                    // If the current tab isn't set, then set
                                    // it.
                                    state.sidebar.tab_kind.set_neq(Some(MenuTabKind::Question));
                                    None
                                }
                            }
                        }))
                    )
                }))
            } else {
                None
            }
        })))
        .child_signal(question.confirm_delete.signal_cloned().map(clone!(state, index, question => move |confirm_delete| {
            log::info!("CONFIRM {confirm_delete}");
            if confirm_delete {
                Some(html!("empty-fragment", {
                    .apply(OverlayHandle::lifecycle(clone!(state, index, question => move || {
                        html!("modal-confirm", {
                            .property("dangerous", true)
                            .property("title", "Warning")
                            .property("content", "Are you sure you want to delete this question?")
                            .property("cancel_text", "Don't delete")
                            .property("confirm_text", "Delete question")
                            .event(clone!(question => move |_evt: events::CustomCancel| question.confirm_delete.set_neq(false)))
                            .event(clone!(state, index, question => move |_evt: events::CustomConfirm| {
                                question.confirm_delete.set_neq(false);
                                state.sidebar.base.delete_question(index.get().unwrap_ji());
                            }))
                        })
                    })))
                }))
            } else {
                None
            }
        })))
    })
}

fn render_tab(tab_kind: MenuTabKind, selected_tab: Mutable<Option<MenuTabKind>>) -> Dom {
    MenuTab::render(
        MenuTab::new(
            tab_kind,
            false,
            true,
            clone!(selected_tab => move || selected_tab.signal_ref(clone!(tab_kind => move |curr| {
                match curr {
                    Some(curr) => *curr == tab_kind,
                    None => false
                }
            }))),
            clone!(tab_kind => move || {
                selected_tab.set_neq(Some(tab_kind));
            }),
        ),
        Some("tabs"),
    )
}

fn render_tab_body(state: Rc<Step3>, tab: Tab) -> Dom {
    match tab {
        Tab::Question => {
            html!("div", {
                .child_signal(current_question_signal(state.clone()).map(clone!(state => move |question| {
                    question.map(clone!(state => move |(index, question)| {
                        let opts = AudioInputOptions::new(Some(
                            question
                                .question_audio
                                .signal_cloned()
                        ));

                        let callbacks = AudioInputCallbacks::new(
                            Some(clone!(state, question => move |audio: Audio| {
                                {
                                    let mut lock = question.question_audio.lock_mut();
                                    *lock = Some(audio);
                                }
                                state.sidebar.base.save_question(index, question.clone());
                            })),
                            Some(clone!(state, question => move || {
                                {
                                    let mut lock = question.question_audio.lock_mut();
                                    *lock = None;
                                }
                                state.sidebar.base.save_question(index, question.clone());
                            })),
                        );

                        html!("empty-fragment", {
                            .child(html!("input-wrapper", {
                                .property("label", crate::strings::step_3::STR_LABEL)
                                .child(html!("textarea" => HtmlTextAreaElement, {
                                    .property_signal("value", question.question_text.signal_cloned().map(|text| text.unwrap_or_default()))
                                    .property("placeholder", crate::strings::step_3::STR_PLACEHOLDER)
                                    .property("rows", 1)
                                    .event(move |evt: events::Change| {
                                        let target = evt.dyn_target::<HtmlTextAreaElement>().unwrap_ji();
                                        let value = target.value();
                                        let value = value.trim();

                                        question.question_text.set(Some(value.into()));

                                        if question.title.get_cloned().is_none() {
                                            question.title.set(Some(value.into()))
                                        }


                                        state.sidebar.base.save_question(index, question.clone());
                                    })
                                }))
                            }))
                            .child(html!("empty-fragment", {
                                .style("display", "block")
                                .style("margin", "20px 0")
                                .child(AudioInput::render(AudioInput::new(
                                    opts,
                                    callbacks,
                                ), None))
                            }))
                        })
                    }))
                })))
            })
        }
        Tab::Answer => html!("empty-fragment", {
            .child(html!("sidebar-empty", {
                .property("label", "Trace all correct answers")
                .property("imagePath", "module/_common/edit/sidebar/illustration-trace-area.svg")
            }))
        }),
    }
}
