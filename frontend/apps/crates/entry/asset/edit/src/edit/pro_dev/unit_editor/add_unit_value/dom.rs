use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::pro_dev::unit::ProDevUnitValue;
use std::path::Path;
use utils::asset::ProDevUnitValueExt;
use utils::events;
use utils::unwrap::UnwrapJiExt;
use web_sys::HtmlElement;

use crate::edit::pro_dev::unit_editor::add_unit_value::{
    add_link::state::AddLink, add_video::state::AddVideo,
};
use crate::edit::pro_dev::unit_editor::UnitValue;

use super::super::add_unit_value::add_file::state::AddFile;

use super::state::AddUnitValue;

impl AddUnitValue {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;

        html!("div" => HtmlElement, {
            .apply_if(slot.is_some(), move |dom| {
                dom.prop("slot", slot)
            })
            .child_signal(state.unit_editor_state.value.signal_cloned().map(clone!(state => move|unit_value| {
                Some(state.render_value_slot(unit_value))
            })))
        })
    }

    fn render_value_slot(self: &Rc<Self>, value_selector: UnitValue) -> Dom {
        let state = Rc::clone(self);

        html!("div" => HtmlElement, {
                    .child({
                        match value_selector {
                            UnitValue::Link(url) => {
                                AddLink::new(Rc::clone(&state), &url).render()
                            },
                            UnitValue::Video(video) => {
                                AddVideo::render(&AddVideo::new(Rc::clone(&state), &video))
                            }
                            UnitValue::File(file) => {
                                match file {
                                    Some(file) => state.render_file_slot(UnitValue::File(Some(file))),
                                    None => AddFile::new(Rc::clone(&state)).render(),
                                }
                            },
                        }
                    })
        })
    }

    pub fn render_file_slot(self: Rc<Self>, file_unit: UnitValue) -> Dom {
        let state = self.clone();
        let file = ProDevUnitValue::try_from(file_unit).unwrap_ji();
        let binding = file.get_link();
        let filename = match get_file_name_from_path(&binding) {
            Some(name) => name,
            None => "",
        };

        html!("unit-edit-value-file", {
            .prop("slot", "file")
            .prop("label", filename)
            .prop("resourceHref", binding)
            .child(html!("fa-button", {
                .prop("slot", "delete")
                .prop("icon", "fa-light fa-trash-can")
                .event(clone!(state => move |_: events::Click| {
                    state.unit_editor_state.value.set(UnitValue::File(None));
                    state.unit_editor_state.changed.set(false)
                }))
            }))
        })
    }
}

fn get_file_name_from_path(path: &str) -> Option<&str> {
    Path::new(path).file_name().and_then(|name| name.to_str())
}
