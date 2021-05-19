use super::state::*;
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
};


impl <Step, Sections, Main, Sidebar, Header, Footer, Overlay> Steps <Step, Sections, Main, Sidebar, Header, Footer, Overlay> 
where
    Step: StepExt + 'static,
    Sections: SectionsExt<Step> + 'static,
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

        if self.sections.allowed_step_change(from, to) {
            if !from.is_preview() {
                self.steps_completed.lock_mut().insert(from);
            }
            self.step.set_neq(to);
        }
    }
    pub fn next_step_allowed_signal(&self) -> impl Signal<Item = Option<bool>> {
        let sections = self.sections.clone();

        self.step.signal()
            .map(clone!(sections => move |from| {
                from.next()
                    .map(|to| {
                        sections.allowed_step_change(from, to)
                    })
            }))
    }
}
