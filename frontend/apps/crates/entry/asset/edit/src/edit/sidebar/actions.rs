use super::course::actions as course_actions;
use super::jig::actions as jig_actions;
use super::pro_dev::actions as pro_dev_actions;
use super::state::*;
use dominator::clone;
use shared::domain::asset::AssetId;
use std::rc::Rc;

pub fn navigate_to_publish(state: Rc<Sidebar>) {
    state.collapsed.set(true);
    match &state.asset_edit_state.asset_id {
        AssetId::JigId(_) => {
            jig_actions::navigate_to_publish(Rc::clone(&state));
        }
        AssetId::CourseId(_) => {
            course_actions::navigate_to_publish(Rc::clone(&state));
        }
        AssetId::ResourceId(_) => unimplemented!(),
        AssetId::ProDevId(_) => {
            pro_dev_actions::navigate_to_publish(Rc::clone(&state));
        }
    }
}

pub fn set_highlight_modules(state: &Rc<Sidebar>, highlight: bool) {
    if highlight {
        state.collapsed.set_neq(false);

        let modules = state.asset_edit_state.sidebar_spots.lock_ref();

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

pub fn set_highlight_units(state: &Rc<Sidebar>, highlight: bool) {
    if highlight {
        state.collapsed.set_neq(false);

        let units = state.asset_edit_state.sidebar_spots.lock_ref();

        if units.iter().filter(|unit| unit.item.is_some()).count() == 0 {
            state
                .highlight_modules
                .set_neq(Some(ModuleHighlight::Publish))
        } else {
            let idx = units.iter().position(|unit| unit.is_incomplete.get());
            match idx {
                Some(idx) => state
                    .highlight_modules
                    .set_neq(Some(ModuleHighlight::Unit(idx))),
                None => {
                    log::info!("New highlight unit");
                    state.highlight_modules.set_neq(None)
                }
            }
        }
    } else {
        state.highlight_modules.set_neq(None);
    }
}

pub fn update_display_name(state: Rc<Sidebar>, value: String) {
    state.loader.load(clone!(state => async move {
        state.asset_edit_state.asset.display_name().set(value.clone());

        match &state.asset_edit_state.asset_id {
            AssetId::JigId(jig_id) => {
                jig_actions::update_display_name(*jig_id, value).await;
            },
            AssetId::CourseId(course_id) => {
                course_actions::update_display_name(*course_id, value).await;
            },
            AssetId::ResourceId(_) => unimplemented!(),
            AssetId::ProDevId(pro_dev_id) => {
                pro_dev_actions::update_display_name(*pro_dev_id, value).await;
            },
        }
    }));
}
