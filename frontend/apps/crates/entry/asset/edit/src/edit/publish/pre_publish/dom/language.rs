use dominator::{clone, html, Dom};
use futures_signals::map_ref;
use futures_signals::signal::{Signal, SignalExt};
use std::rc::Rc;
use utils::{events, languages::Language};

use super::super::state::PrePublish;

const STR_LANGUAGE_LABEL: &str = "Language of instruction";
const STR_LANGUAGE_PLACEHOLDER: &str = "Select language";

impl PrePublish {
    pub fn render_languages(self: Rc<Self>) -> Dom {
        let state = Rc::clone(&self);
        html!("input-select", {
            .prop("slot", "language")
            .prop("label", STR_LANGUAGE_LABEL)
            .prop("placeholder", STR_LANGUAGE_PLACEHOLDER)
            .prop_signal("value", language_value_signal(state.clone()))
            .prop_signal("error", {
                (map_ref! {
                    let submission_tried = state.submission_tried.signal(),
                    let value = state.asset.language().signal_cloned()
                        => (*submission_tried, value.clone())
                })
                    .map(|(submission_tried, value)| {
                        submission_tried && value.is_empty()
                    })
            })
            .children(state.languages.iter().map(clone!(state => move |language| {
                render_language(language, state.clone())
            })))
        })
    }
}

fn render_language(Language(lang_code, land_label): &Language, state: Rc<PrePublish>) -> Dom {
    html!("input-select-option", {
        .text(land_label)
        .prop_signal("selected", state.asset.language().signal_cloned().map(clone!(lang_code => move |selected_lang| {
            lang_code == selected_lang
        })))
        .event(clone!(state, lang_code => move |evt: events::CustomSelectedChange| {
            if evt.selected() {
                state.asset.language().set(lang_code.to_string());
                state.save_draft();
            }
        }))
    })
}

fn language_value_signal(state: Rc<PrePublish>) -> impl Signal<Item = &'static str> {
    state.asset.language().signal_cloned().map(clone!(state => move |selected_language| {
        match state.languages.iter().find(|Language(lang_code, _)| lang_code == &selected_language) {
            Some(lang) => lang.1,
            None => ""
        }
    }))
}
