use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;

use shared::domain::pro_dev::unit::ProDevUnitValue;
use web_sys::HtmlElement;

use crate::edit::pro_dev::unit_editor::add_unit_value::add_link::state::AddLink;
use crate::edit::pro_dev::unit_editor::add_unit_value::add_video::state::AddVideo;
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
                            _ => {
                                AddFile::new(Rc::clone(&state)).render()
                            },
                        }
                    })
        })
    }
}
