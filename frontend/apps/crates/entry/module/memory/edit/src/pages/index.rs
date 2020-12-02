use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader, dynamic_class_signal};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use utils::components::module_page::*;
use async_trait::async_trait;
use super::{
    choose_mode,
    main::Main, 
    step_1_sidebar::Step1Sidebar,
    step_2_sidebar::Step2Sidebar,
    step_3_sidebar::Step3Sidebar,
    step_4_sidebar::Step4Sidebar,
    footer::Footer,
    header::Header,
};
pub struct IndexPage {
    pub state: Rc<State>,
}

impl IndexPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let state = Rc::new(State::new(jig_id, module_id));
        Rc::new(Self { state })
    }
}

#[async_trait(?Send)]
impl ModuleRenderer for IndexPage {
    type Data = Option<raw::GameData>;

    async fn load(_self:Rc<Self>) -> Option<raw::GameData> { 
        if let Some(raw_data) = debug::settings().data {
            Some(raw_data)
        } else {
            raw::GameData::load(_self.state.jig_id.clone(), _self.state.module_id.clone()).await
        }
    }

    fn render(_self: Rc<Self>, data: Option<raw::GameData>) -> ModuleRenderOutput {
        _self.state.set_from_raw(data);
        ModuleRenderOutput::new_empty(Some(render_loaded(_self.state.clone())))
    }
}

fn render_loaded(state: Rc<State>) -> Dom {
    html!("div", {
        .child_signal(state.game_mode.signal().map(clone!(state => move |mode| {
            match mode {
                None => Some(choose_mode::render(state.clone())),
                Some(mode) => Some(ModulePage::render(ModulePage::new(StaticModuleRenderer::new(
                    ModePage::render(ModePage::new(state.clone(), mode))
                ))))
            }
        })))
    })
}

pub struct ModePage {
    pub state: Rc<State>,
    pub mode: GameMode
}

impl ModePage {
    pub fn new(state:Rc<State>, mode: GameMode) -> Rc<Self> {
        Rc::new(Self { 
            state,
            mode,
        })
    }
    pub fn render(_self: Rc<Self>) -> ModuleRenderOutput {
        ModuleRenderOutput {
            kind: ModulePageKind::EditPlain,
            sidebar: Some(Self::render_sidebar(_self.clone())),
            main: Some(Main::render(Main::new(_self.state.clone(), _self.mode))),
            footer: Some(Footer::render(Footer::new(_self.state.clone(), _self.mode))),
            header: Some(Header::render(Header::new(_self.state.clone(), _self.mode))),
        }
    }

    fn render_sidebar(_self: Rc<Self>) -> Dom {
        html!("div", {
            .child_signal(_self.state.step.signal().map(clone!(_self => move |step| Some(
                match step {
                    Step::One => {
                        Step1Sidebar::render(Step1Sidebar::new(_self.state.clone(), _self.mode))
                    },
                    Step::Two => {
                        Step2Sidebar::render(Step2Sidebar::new(_self.state.clone(), _self.mode))
                    },
                    Step::Three => {
                        Step3Sidebar::render(Step3Sidebar::new(_self.state.clone(), _self.mode))
                    },
                    Step::Four => {
                        Step4Sidebar::render(Step4Sidebar::new(_self.state.clone(), _self.mode))
                    },
                }
            ))))
        })
    }

}
