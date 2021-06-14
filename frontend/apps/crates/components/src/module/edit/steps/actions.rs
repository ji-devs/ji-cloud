use super::state::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};
use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};

impl <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> Steps <RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay> 
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
    pub fn try_next_step(&self) {
        if let Some(to) = self.step.get().next() {
            self.try_change_step(to);
        }
    }
    pub fn try_change_step(&self, to: Step) {
        let from = self.step.get();

        if self.base.allowed_step_change(from, to) {
            if !from.is_preview() {
                self.steps_completed.lock_mut().insert(from);

                self.history.push_modify(|raw| {
                    raw.set_editor_state_step(to);
                    raw.insert_editor_state_step_completed(from);   
                });
            }
            self.step.set_neq(to);
        }
    }
}
