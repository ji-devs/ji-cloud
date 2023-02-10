use super::super::{nav::dom::render_nav, state::*};
use awsm_web::loaders::fetch::fetch_url;
use dominator::{clone, html, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use serde::Deserialize;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::JsValue;

use crate::{jigzi_help::JigziHelp, module::_common::edit::header::controller::dom::ControllerDom};

use shared::domain::module::body::{BodyExt, ModeExt, StepExt};
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

    add_slot_to_dom(dom, "main")
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
        .prop("slot", "sidebar")
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
        utils::path::config_cdn_url(format!("module/_header/tips/{}.json", module_kind));

    #[derive(Debug, Deserialize, Default, Clone)]
    struct HeaderConfig {
        title: String,
        steps: Vec<HeaderConfigStep>,
    }
    #[derive(Debug, Deserialize, Default, Clone)]
    struct HeaderConfigStep {
        #[serde(default)]
        #[serde(rename = "default")]
        default_content: Option<HeaderConfigTab>,
        #[serde(default)]
        tabs: HashMap<String, HeaderConfigTab>,
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
            let tab_kind = state.sidebar.tab_kind()
            => {
                let step_index = step.as_number() - 1;
                let tab_config = header_config.steps.get(step_index)
                    .and_then(|step_config| {
                        match tab_kind {
                            Some(tab_kind) => {
                                step_config.tabs
                                    .get(&format!("{}", tab_kind))
                                    .or(step_config.default_content.as_ref())
                            }
                            None => step_config.default_content.as_ref(),
                        }
                    });

                tab_config.cloned()
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
                Err(_) => Default::default()
            };

            header_config.set(data);
        }))
        .prop("slot", "header")
        .prop_signal("headerTitle", {
            header_config.signal_ref(|h| {
                h.title.clone()
            })
        })
        .prop("subtitle", state.mode.map_or(JsValue::UNDEFINED, |m| JsValue::from_str(m.label())))
        .child_signal(tab_config_sig().map(|tab| {
            match tab {
                Some(tab) => {
                    let HeaderConfigTab {title, body} = tab;

                    if !title.is_empty() && !body.is_empty() {
                        Some(
                            JigziHelp::new(
                                title,
                                body,
                                "module-header"
                            )
                            .render(Some("help"), Rc::new(None::<fn() -> Dom>))
                        )
                    } else {
                        None
                    }
                }
                None => None
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
    let change_to = Mutable::new(None);

    let should_change_to = map_ref! {
        let change_to = change_to.signal_cloned(),
        let can_continue_next = state.base.can_continue_next()
            => {
                match (change_to, can_continue_next) {
                    (Some(to), true) => Some(*to),
                    _ => None
                }
            }
    };

    html!("module-footer", {
        .future(should_change_to.for_each(clone!(state => move |to| {
            if let Some(to) = to {
                if !state.base.continue_next() {
                    state.try_change_step(to);
                }
            }
            async {}
        })))
        .prop("slot", "footer")
        .child(Footer::render(state.footer.clone()))
        .child(html!("module-footer-continue-button", {
            .prop("slot", "btn")
            .prop_signal("enabled", state.base.can_continue_next())
            .event(clone!(state => move |_evt: events::Next| {
                change_to.set(state.step.get().next());
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
        .prop("slot", slot)
        .style("display", "contents")
        .child(dom)
    })
}
