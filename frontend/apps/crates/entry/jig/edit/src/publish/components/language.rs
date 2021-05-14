use std::rc::Rc;
use dominator::{Dom, clone, html};
use futures_signals::signal::{Signal, SignalExt};
use utils::events;

use crate::publish::state::{Language, State};


const STR_LANGUAGE_LABEL: &'static str = "Language of instructions";
const STR_LANGUAGE_PLACEHOLDER: &'static str = "Select language";

pub fn render(state: Rc<State>) -> Dom {
    html!("dropdown-select", {
        .property("slot", "language")
        .property("label", STR_LANGUAGE_LABEL)
        .property("placeholder", STR_LANGUAGE_PLACEHOLDER)
        .property_signal("value", language_value_signal(state.clone()))
        .children(state.languages.iter().map(clone!(state => move |language| {
            render_language(language, state.clone())
        })))
    })
}

fn render_language((lang_code, land_label): &Language, state: Rc<State>) -> Dom {
    html!("li-check", {
        .text(&land_label)
        .property_signal("selected", state.jig.language.signal_cloned().map(clone!(lang_code => move |selected_lang| {
            lang_code == &selected_lang
        })))
        .event(clone!(state, lang_code => move |_: events::Click| {
            state.jig.language.set(lang_code.to_string());
        }))
    })
}

fn language_value_signal(state: Rc<State>) -> impl Signal<Item = &'static str> {
    state.jig.language.signal_cloned().map(clone!(state => move |selected_language| {
        match state.languages.iter().find(|(lang_code, _)| lang_code == &selected_language) {
            Some(lang) => lang.1,
            None => ""
        }
    }))
}
