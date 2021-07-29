use wasm_bindgen::prelude::*;
use crate::tooltip::state::{State as TooltipState};
use super::dom::ColumnSide;
use std::rc::Rc;

pub struct Callbacks {
    //this is passed to the JS custom element
    pub constrain: Closure<dyn Fn(String) -> String>,

    pub set_tooltip_error: Box<dyn Fn(Option<Rc<TooltipState>>)>,
    pub replace_list: Box<dyn Fn(Vec<(String, String)>)>,

    //row, col
    pub get_placeholder: Box<dyn Fn(usize, usize) -> Option<String>>,
    pub get_header: Box<dyn Fn(ColumnSide) -> String>,
}

// Example: 
// state.app.limit_text(crate::config::SINGLE_LIST_CHAR_LIMIT, text)

impl Callbacks {
    pub fn new(
        constrain: impl Fn(String) -> String + 'static,
        set_tooltip_error: impl Fn(Option<Rc<TooltipState>>) + 'static,
        replace_list: impl Fn(Vec<(String, String)>) + 'static,
        get_placeholder: impl Fn(usize, usize) -> Option<String> + 'static,
        get_header: impl Fn(ColumnSide) -> String + 'static,
    ) -> Self {
        Self {
            constrain: Closure::wrap(Box::new(constrain)),
            set_tooltip_error: Box::new(set_tooltip_error),
            replace_list: Box::new(replace_list),
            get_placeholder: Box::new(get_placeholder),
            get_header: Box::new(get_header),
        }
    }
}
