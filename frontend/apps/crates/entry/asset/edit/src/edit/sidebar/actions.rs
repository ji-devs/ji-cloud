use super::course::actions as course_actions;
use super::jig::actions as jig_actions;
use super::state::*;
use dominator::clone;
use shared::domain::asset::Asset;
use std::rc::Rc;

pub fn navigate_to_publish(state: Rc<State>) {
    state.collapsed.set(true);
    match &state.asset {
        Asset::Jig(_) => {
            jig_actions::navigate_to_publish(Rc::clone(&state));
        }
        Asset::Course(_) => {
            course_actions::navigate_to_publish(Rc::clone(&state));
        }
        Asset::Resource(_) => unimplemented!(),
    }
}

pub fn set_highlight_modules(state: &Rc<State>, highlight: bool) {
    if highlight {
        state.collapsed.set_neq(false);

        let modules = state.modules.lock_ref();

        if modules
            .iter()
            .filter(|module| module.item.is_some())
            .count()
            == 0
        {
            state
                .highlight_modules
                .set_neq(Some(ModuleHighlight::Publish))
        } else {
            let idx = modules.iter().position(|module| module.is_incomplete.get());
            match idx {
                Some(idx) => state
                    .highlight_modules
                    .set_neq(Some(ModuleHighlight::Module(idx))),
                None => state.highlight_modules.set_neq(None),
            }
        }
    } else {
        state.highlight_modules.set_neq(None);
    }
}

pub fn update_display_name(state: Rc<State>, value: String) {
    state.loader.load(clone!(state => async move {
        state.name.set(value.clone());

        match &state.asset {
            Asset::Jig(jig) => {
                jig_actions::update_display_name(jig.id, value).await;
            },
            Asset::Course(_) => todo!(),
            Asset::Resource(_) => unimplemented!(),
        }
    }));
}
