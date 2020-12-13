use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::UnwrapThrowExt;
use wasm_bindgen::JsCast;
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, Signal, always},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    CancelableFutureHandle, 
};
use dominator::{DomBuilder, Dom, html, events, clone, apply_methods};
use dominator_helpers::{elem, with_data_id};
use awsm_web::dom::*;
use wasm_bindgen_futures::{JsFuture, spawn_local, future_to_promise};
use futures::future::ready;
use crate::templates;
use components::module::page::*;
use std::pin::Pin;
use web_sys::{HtmlElement, Element, HtmlInputElement, HtmlTemplateElement, DocumentFragment, Document};

pub struct Page { }

impl Page {
    pub fn render() -> Dom {
        render_steps() 
    }
}

struct State {
    pub current_step: Mutable<u32>
}

impl State {
    pub fn new (initial_step:u32) -> Rc<Self> {
        Rc::new(Self {
            current_step: Mutable::new(initial_step)
        })
    }
}

pub fn render_steps() -> Dom {
    let state = State::new(1);

    html!("steps-nav", {
        .property("steps", 4)
        .children(vec![
            render_button(1, "This", state.clone()),
            render_button(2, "Is", state.clone()),
            render_button(3, "A", state.clone()),
            render_button(4, "Test", state.clone()),
        ])
    })
}

fn render_button(step:u32, label:&str, state:Rc<State>) -> Dom {
    html!("circle-button", {
        .property("text", format!("{}", step))
        .property("label", label)
        .property("slot", format!("btn-{}", step))
        .property_signal("active", state.current_step.signal().map(move |current_step| {
            if current_step == step {
                true
            } else {
                false
            }
        }))
        .event(clone!(step, state => move |evt:events::Click| {
            state.current_step.set(step);
        }))
    })
}
