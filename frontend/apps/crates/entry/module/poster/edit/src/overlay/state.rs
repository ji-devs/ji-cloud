use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable,  SignalExt, Signal},
    signal_vec::{MutableVec, SignalVecExt, SignalVec},
    CancelableFutureHandle, 
};
use dominator::{Dom, clone};
use std::rc::Rc;
use components::tooltip::{
    dom::TooltipDom,
    types::TooltipData,
};

pub struct State {
    pub tooltips: Tooltips
}

pub struct Tooltips {
    pub delete: Mutable<Option<TooltipData>>,
    pub list_error: Mutable<Option<TooltipData>>
}
impl Tooltips {
    pub fn new() -> Self {
        Self {
            delete: Mutable::new(None),
            list_error: Mutable::new(None),
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            tooltips: Tooltips::new() 
        }
    }

    pub fn children(&self) -> impl SignalVec<Item = Dom> {
        let sig = map_ref! {
            let delete = self.tooltips.delete.signal_cloned(),
            let list_error = self.tooltips.list_error.signal_cloned()
            => {
                let mut children:Vec<Dom> = Vec::new();
                if let Some(delete) = delete.as_ref() {
                    children.push(TooltipDom::render(delete.clone()));
                }
                if let Some(list_error) = list_error.as_ref() {
                    log::info!("HMMMMM");
                    children.push(TooltipDom::render(list_error.clone()));
                }

                children
            }
        };
        sig.to_signal_vec()
    }
}
