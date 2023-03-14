use std::rc::Rc;

use components::overlay::handle::OverlayHandle;
use dominator::traits::AsStr;
use dominator::{clone, html, with_node, Dom};
use futures_signals::signal::SignalExt;

use shared::api::endpoints::pro_dev::unit;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement};

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
            .with_node!(elem => {
                .child_signal(state.unit_editor_state.value_type.signal().map(clone!(state, elem => move|unit_type| {
                    unit_type.map(|unit_type| state.render_value_slot(unit_type))
                })))
            })

        })
    }

    fn render_value_slot(self: &Rc<Self>, value_selector: UnitValueType) -> Dom {
        let state = Rc::clone(self);
        html!("div" => HtmlElement, {
                    .child({
                        log::info!("selector {:?}", value_selector);

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
