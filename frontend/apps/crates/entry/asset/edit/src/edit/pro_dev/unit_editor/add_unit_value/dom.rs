use std::rc::Rc;

use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;

use web_sys::HtmlElement;

use crate::edit::pro_dev::unit_editor::add_unit_value::add_link::state::AddLink;
use crate::edit::pro_dev::unit_editor::UnitValueType;

use super::super::add_unit_value::add_file::state::AddFile;

use super::state::AddUnitValue;

impl AddUnitValue {
    pub fn render(self: Rc<Self>, slot: Option<&str>) -> Dom {
        let state = self;

        html!("div" => HtmlElement, {
            .apply_if(slot.is_some(), move |dom| {
                dom.prop("slot", slot)
            })
            .child_signal(state.unit_editor_state.value_type.signal().map(clone!(state => move|unit_type| {
                unit_type.map(|unit_type| state.render_value_slot(unit_type))
            })))

        })
    }

    fn render_value_slot(self: &Rc<Self>, value_selector: UnitValueType) -> Dom {
        let state = Rc::clone(self);
        html!("div" => HtmlElement, {
                    .child({
                        match value_selector {
                            UnitValueType::Link => {
                                AddLink::new(Rc::clone(&state)).render()
                            },
                            UnitValueType::File => {
                                AddFile::new(Rc::clone(&state)).render()
                            },
                            UnitValueType::Youtube => todo!()
                        }
                    })
        })
    }
}
