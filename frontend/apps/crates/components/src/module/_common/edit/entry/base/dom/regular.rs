use super::super::{nav::dom::render_nav, state::*};
use awsm_web::loaders::fetch::fetch_url;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use serde::Deserialize;
use wasm_bindgen::JsValue;
use std::rc::Rc;

use crate::{jigzi_help::JigziHelp, module::_common::edit::header::controller::dom::ControllerDom};

use shared::domain::jig::module::body::{BodyExt, ModeExt, StepExt};
use utils::prelude::*;

pub fn render_main_bg<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    let dom = match Main::render_bg(state.main.clone()) {
        Some(dom) => dom,
        None => html!("empty-fragment"),
    };

    add_slot_to_dom(dom, "main-bg")
}
pub fn render_main<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    add_slot_to_dom(Main::render(state.main.clone()), "main")
}

pub fn render_sidebar<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    html!("module-sidebar", {
        .property("slot", "sidebar")
        .child(render_nav(state.clone()))
        .child(add_slot_to_dom(Sidebar::render(state.sidebar.clone()), "content"))
    })
}

pub fn render_header<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    let module_kind = RawData::kind().as_str();

    //TODO - load from localization endpoint
    let str_config_url =
        utils::path::config_cdn_url(format!("module/_header/{}.json", module_kind));

    #[derive(Deserialize, Default, Clone)]
    struct HeaderConfig {
        title: String,
        steps: Vec<HeaderConfigStep>,
    }
    #[derive(Deserialize, Default, Clone)]
    struct HeaderConfigStep {
        tabs: Vec<HeaderConfigTab>,
    }
    #[derive(Debug, Deserialize, Default, Clone)]
    struct HeaderConfigTab {
        title: String,
        body: String,
    }

    let header_config = Mutable::new(HeaderConfig::default());

    let tab_config_sig = clone!(header_config, state => move || {
        map_ref! {
            let header_config = header_config.signal_cloned(),
            let step = state.step.signal_cloned(),
            let tab_index = state.sidebar.tab_index()
            => {

                let step_index = step.as_number() - 1;
                let tab_index = tab_index.unwrap_or_default();

                //log::info!("step: {}, tab: {}", step_index, tab_index);

                let tab_config = header_config.steps.get(step_index)
                    .and_then(|step_config| {
                        step_config.tabs.get(tab_index)
                    } );

                tab_config.cloned().unwrap_or_default()
            }
        }
    });

    html!("module-header", {
        .future(clone!(header_config => async move {
            let data:HeaderConfig = match fetch_url(&str_config_url).await {
                Ok(resp) => {
                    resp
                        .json_from_str()
                        .await
                        .unwrap_or_default()
                },
                Err(_) => {
                    HeaderConfig::default()
                }
            };

            header_config.set(data);
        }))
        .property("slot", "header")
        .property_signal("headerTitle", {
            header_config.signal_ref(|h| {
                h.title.clone()
            })
        })
        .property("subtitle", state.mode.map_or(JsValue::UNDEFINED, |m| JsValue::from_str(m.label())))
        .child_signal(tab_config_sig().map(|tab| {
            let HeaderConfigTab {title, body} = tab;

            if !title.is_empty() && !body.is_empty() {
                Some(
                    JigziHelp::new(
                        title,
                        body,
                        "module-header"
                    )
                    .render(Some("help"))
                )
            } else {
                None
            }
        }))
        .child(ControllerDom::render(
            state.history.clone(),
            clone!(state => move || {
                state.try_change_step(Step::get_preview());
            })
        ))
        .child(Header::render(state.header.clone()))
    })
}

pub fn render_footer<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    html!("module-footer", {
        .property("slot", "footer")
        .child(Footer::render(state.footer.clone()))
        .child(html!("module-footer-continue-button", {
            .property("slot", "btn")
            .property_signal("enabled", state.base.next_step_allowed_signal())
            .event(clone!(state => move |_evt:events::Next| {
                state.try_next_step();
            }))
        }))
    })
}

pub fn render_overlay<RawData, Mode, Step, Base, Main, Sidebar, Header, Footer, Overlay>(
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
    add_slot_to_dom(Overlay::render(state.overlay.clone()), "overlay")
}

fn add_slot_to_dom(dom: Dom, slot: &str) -> Dom {
    //there might be a better way, like Dom->DomBuilder->Dom
    html!("empty-fragment", {
        .property("slot", slot)
        .style("display", "contents")
        .child(dom)
    })
}
