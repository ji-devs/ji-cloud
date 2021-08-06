use super::super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen::prelude::*;

pub fn render_nav<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Dom
where
    RawData: BodyExt<Mode, Step> + 'static,
    Mode: ModeExt + 'static,
    Step: StepExt + 'static,
    Base: BaseExt<Step> + 'static,
    Main: MainExt + 'static,
    Sidebar: SidebarExt + 'static,
    Header: HeaderExt + 'static,
    Footer: FooterExt + 'static,
    Overlay: OverlayExt + 'static,
{
    html!("steps-nav", {
        .property("slot", "nav")
        .children(
            Step::get_list()
                .into_iter()
                .map(clone!(state => move |step| {
                    html!("step-nav", {
                        .property("number", JsValue::from_f64(step.as_number() as f64))
                        .property("label", step.label())
                        .property_signal("active", state.step.signal().map(move |curr| {
                            if curr == step {
                                true
                            } else {
                                false
                            }
                        }))
                        .property_signal("completed", state.steps_completed.signal_ref(move |steps_completed| {
                            steps_completed.contains(&step)
                        }))
                        .event(clone!(state => move |_evt:events::Click| {
                            state.try_change_step(step);
                        }))
                    })
                }))
                .collect::<Vec<Dom>>()
        )
    })
}
