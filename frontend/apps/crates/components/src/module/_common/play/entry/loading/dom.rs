use super::super::state::*;
use dominator::{html, Dom};
use std::rc::Rc;

use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};

pub fn render_loading<RawData, Mode, Step, Base>(
    _state: Rc<GenericState<RawData, Mode, Step, Base>>,
) -> Dom
where
    Base: BaseExt + 'static,
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
{
    //TODO - make this a custom element with some visual indicator of loading progress
    html!("div", {
        .style("position", "absolute")
        .style("width", "100%")
        .style("height", "100%")
        .property("slot", "main")
    })
}
