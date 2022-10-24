mod preview;
use preview::*;

mod regular;
use regular::*;

use super::state::*;
use dominator::Dom;
use std::rc::Rc;

use shared::domain::{
    asset::AssetId,
    module::{
        body::{BodyExt, ModeExt, StepExt},
        ModuleId,
    },
};

pub fn render<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
    asset_id: AssetId,
    module_id: ModuleId,
    state: Rc<AppBase<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>>,
) -> Vec<Dom>
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
    match state.step.get().is_preview() {
        true => match state.jig_is_post_preview.get() {
            false => {
                vec![
                    render_preview_header(RawData::kind(), state.clone()),
                    render_preview_main(asset_id, module_id, state.clone()),
                    render_preview_overlay(RawData::kind(), asset_id, module_id, state),
                ]
            }
            true => {
                vec![render_preview_overlay(
                    RawData::kind(),
                    asset_id,
                    module_id,
                    state,
                )]
            }
        },
        false => {
            vec![
                render_main_bg(state.clone()),
                render_main(state.clone()),
                render_sidebar(state.clone()),
                render_header(state.clone()),
                render_footer(state.clone()),
                render_overlay(state),
            ]
        }
    }
}
