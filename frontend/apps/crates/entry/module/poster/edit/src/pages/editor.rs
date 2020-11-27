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
use dominator_helpers::{elem, with_data_id, spawn_future, AsyncLoader};
use crate::templates;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::data::*;
use crate::debug;
use utils::components::module_page::*;
use async_trait::async_trait;

pub struct EditorPage {
    pub game_state: Rc<GameState>,
}

impl EditorPage {
    pub fn new(jig_id: String, module_id: String) -> Rc<Self> {
        let game_state = Rc::new(GameState::new(jig_id, module_id));
        Rc::new(Self { game_state })
    }
}

#[async_trait(?Send)]
impl ModuleRenderer for EditorPage {
    type Data = raw::Poster;

    async fn load(_self:Rc<Self>) -> raw::Poster { 
        if let Some(raw_poster) = debug::settings().poster {
            raw_poster
        } else {
            log::info!("loading...");
            raw::Poster::load(_self.game_state.jig_id.clone(), _self.game_state.module_id.clone()).await
        }
    }

    fn render(_self: Rc<Self>, data: raw::Poster) -> ModuleRenderOutput {
        _self.game_state.set_from_loaded(data);
        ModuleRenderOutput {
            kind: ModulePageKind::EditResize,
            sidebar: Some(Self::render_sidebar(_self.game_state.clone())),
            header: Some(Self::render_header(_self.game_state.clone())),
            main: Some(Self::render_main(_self.game_state.clone())),
            footer: Some(Self::render_footer(_self.game_state.clone())),
        }
    }

}

impl EditorPage {
    fn render_sidebar(state:Rc<GameState>) -> Dom {
        elem!(templates::sidebar(), {})
    }
    fn render_header(state:Rc<GameState>) -> Dom {
        elem!(templates::header("Create a Cover Page", "Introduce your topic<br/>Use the blue panel for selecting layouts, themes, and adding content"), {})
    }
    fn render_main(state:Rc<GameState>) -> Dom {
        elem!(templates::main(), {})
    }
    fn render_footer(state:Rc<GameState>) -> Dom {
        elem!(templates::footer(), {})
    }
}
