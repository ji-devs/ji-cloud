use super::callbacks::*;
use crate::module::_groups::cards::{edit::state::*, lookup::Side};

use futures_signals::signal::{Mutable, ReadOnlyMutable};
use shared::domain::jig::module::body::_groups::cards::Step;
use std::{cell::RefCell, rc::Rc};
use web_sys::HtmlElement;

/// Used for passing it's contents as properties to the modal-confirm element
#[derive(Clone)]
pub struct ModalAction {
    /// Title of the modal
    pub title: &'static str,
    /// Content of the modal
    pub content: &'static str,
    /// Confirm button text
    pub confirm: &'static str,
    /// Cancel button text
    pub cancel: &'static str,
    /// Callback handler called when the confirm button is clicked
    pub handler: Rc<dyn Fn()>,
}

impl ModalAction {
    pub fn new(
        title: &'static str,
        content: &'static str,
        confirm: &'static str,
        cancel: &'static str,
        handler: Rc<dyn Fn()>
    ) -> Self {
        Self {
            title,
            content,
            confirm,
            cancel,
            handler,
        }
    }
}

pub struct MainCard<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub step: Step,
    pub index: ReadOnlyMutable<Option<usize>>,
    pub side: Side,
    pub card: Card,
    pub other: Card,
    pub input_ref: Rc<RefCell<Option<HtmlElement>>>,
    pub editing_active: Mutable<bool>,
    pub is_image: bool,
    pub is_hovering: Mutable<bool>,
    /// Whether the context menu is currently open
    pub menu_open: Mutable<bool>,
    /// A reference to the HtmlElement which holds the context menu items
    pub menu_container_elem: Mutable<Option<HtmlElement>>,
    /// A signal to render the confirm modal with the provided properties
    pub confirm_action: Mutable<Option<Rc<ModalAction>>>,
    pub callbacks: CardCallbacks,
}

impl<RawData: RawDataExt, E: ExtraExt> MainCard<RawData, E> {
    pub fn new(
        base: Rc<CardsBase<RawData, E>>,
        step: Step,
        index: ReadOnlyMutable<Option<usize>>,
        side: Side,
        card: Card,
        other: Card,
    ) -> Rc<Self> {
        let is_image = match card.card_content {
            CardContent::Image(_) => true,
            _ => false,
        };

        let callbacks = {
            if is_image {
                CardCallbacks::new(Some(|| {}))
            } else {
                CardCallbacks::new(None::<fn()>)
            }
        };
        Rc::new(Self {
            base,
            step,
            index,
            side,
            card,
            other,
            input_ref: Rc::new(RefCell::new(None)),
            editing_active: Mutable::new(false),
            is_image,
            is_hovering: Mutable::new(false),
            menu_open: Mutable::new(false),
            menu_container_elem: Mutable::new(None),
            confirm_action: Mutable::new(None),
            callbacks,
        })
    }
}
