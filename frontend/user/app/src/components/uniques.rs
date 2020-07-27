use wasm_bindgen::prelude::*;
use gloo_events::EventListener;
use web_sys::HtmlInputElement;
use std::collections::VecDeque;
use shipyard::*;
use derive_more::{Deref, DerefMut};

pub use core::routes::Route;
pub use crate::utils::templates::TemplateManager;

pub struct DomRoot(pub web_sys::Element);

impl ::std::ops::Deref for DomRoot {
    type Target = web_sys::Element;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ::std::ops::DerefMut for DomRoot {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


#[derive(Clone, Copy)]
pub enum SigninState {
    None,
    Signin,
    Complete
}
pub struct SigninListeners(pub Vec<EventListener>);
