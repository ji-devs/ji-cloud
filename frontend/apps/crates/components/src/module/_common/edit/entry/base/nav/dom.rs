use super::super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use shared::domain::module::body::{BodyExt, ModeExt, StepExt};
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
    let dense = Step::get_list().len() >= 5;
    html!("steps-nav", {
        .prop("slot", "nav")
        .prop("dense", dense)
        .children(
            Step::get_list()
                .into_iter()
                .map(clone!(state => move |step| {
                    html!("step-nav", {
                        .prop("dense", dense)
                        .prop("number", JsValue::from_f64(step.as_number() as f64))
                        .prop("label", step.label())
                        .prop_signal("active", state.step.signal().map(move |curr| {
                            curr == step
                        }))
                        .prop_signal("completed", state.steps_completed.signal_ref(move |steps_completed| {
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
