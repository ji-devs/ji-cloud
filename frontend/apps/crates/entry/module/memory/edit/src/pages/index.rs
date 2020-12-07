use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVecExt},
    CancelableFutureHandle, 
};
use web_sys::{HtmlElement, Element, HtmlInputElement};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{
    elem, 
    with_data_id, 
    dynamic_class_signal,
    signals::{OptionSignal, EitherSignal}
};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use components::module::page::*;
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

type LoadedData = (String, String, Option<raw::GameData>);

pub fn render(jig_id: String, module_id: String) -> Dom {
    ModulePage::<IndexPageRenderer, _>::render(move || async move {
        if let Some(raw_data) = debug::settings().data {
            (jig_id, module_id, Some(raw_data))
        } else {
            let raw_data = raw::GameData::load(jig_id.clone(), module_id.clone()).await.ok();
            (jig_id, module_id, raw_data)
        }
    })
}

struct IndexPageRenderer {
    pub state: Rc<State>
}

impl ModuleRenderer for IndexPageRenderer {
    type Data = LoadedData;
    type PageKindSignal = impl Signal<Item = ModulePageKind>;
    type SidebarSignal = impl Signal<Item = Option<Dom>>;
    type HeaderSignal = impl Signal<Item = Option<Dom>>;
    type MainSignal = impl Signal<Item = Option<Dom>>;
    type FooterSignal = impl Signal<Item = Option<Dom>>;

    fn new((jig_id, module_id, raw_data):LoadedData) -> Self {
        Self { 
            state: State::new(jig_id, module_id, raw_data)
        }
    }
    fn page_kind_signal(_self: Rc<Self>) -> Self::PageKindSignal {
        _self.state.game_mode.signal().map(|mode| {
            match mode {
                None => ModulePageKind::Empty,
                Some(_) => ModulePageKind::EditPlain
            }
        })
    }

    fn sidebar_signal(_self: Rc<Self>) -> Self::SidebarSignal { 
        let state = _self.state.clone();
        map_ref!{
            let game_mode = _self.state.game_mode.signal(),
            let step = _self.state.step.signal()
            => move {
                game_mode.map(|game_mode| {
                    log::info!("{:?}", step);

                    match step {
                        Step::One => {
                            Step1Sidebar::render(Step1Sidebar::new(state.clone(), game_mode))
                        },
                        Step::Two => {
                            Step2Sidebar::render(Step2Sidebar::new(state.clone(), game_mode))
                        },
                        Step::Three => {
                            Step3Sidebar::render(Step3Sidebar::new(state.clone(), game_mode))
                        },
                        Step::Four => {
                            Step4Sidebar::render(Step4Sidebar::new(state.clone(), game_mode))
                        },
                    }
                })
            }
        }
    }

    fn header_signal(_self: Rc<Self>) -> Self::HeaderSignal { 
        _self.state.game_mode.signal().map(clone!(_self => move |game_mode| {
            OptionSignal::new(game_mode.map(|game_mode| {
                Header::render(_self.state.clone(), game_mode)
            }))
        }))
        .flatten()
    }

    fn main_signal(_self: Rc<Self>) -> Self::MainSignal { 
        use std::pin::Pin;

        _self.state.game_mode.signal()
            .map(clone!(_self => move |game_mode| {
                let state = _self.state.clone();
                
                match game_mode {
                    None => EitherSignal::Left(always(Some(
                            choose_mode::render(state.clone())
                        ))),
                    Some(game_mode) => 
                        EitherSignal::Right(
                            Main::render(state.clone(), game_mode)
                                .map(|main| Some(main))
                        )
                }
            }))
            .flatten()
    }
    fn footer_signal(_self: Rc<Self>) -> Self::FooterSignal { 

        _self.state.game_mode.signal().map(clone!(_self => move |game_mode| {
            OptionSignal::new(game_mode.map(|game_mode| {
                Footer::render(_self.state.clone(), game_mode)
            }))
            .map(|x| x.flatten())
        }))
        .flatten()
    }
}