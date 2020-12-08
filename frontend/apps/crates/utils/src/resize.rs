/* See https://codepen.io/dakom/pen/WNxYrQM */
/* This is slightly adapted to work on a containing element instead of window */

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, HtmlElement, Element};
use config::{STAGE_WIDTH, STAGE_HEIGHT, STAGE_PADDING_X_PERC, STAGE_PADDING_Y_PERC, STAGE_RATIO};
use once_cell::sync::OnceCell;
use std::sync::atomic::{Ordering, AtomicI32};

static CONTAINER_WIDTH: AtomicI32 = AtomicI32::new(0);
static CONTAINER_HEIGHT: AtomicI32 = AtomicI32::new(0);
static CONTAINER_X: AtomicI32 = AtomicI32::new(0);
static CONTAINER_Y: AtomicI32 = AtomicI32::new(0);

#[derive(Copy, Clone, Debug)]
pub struct ModuleBounds { }

impl ModuleBounds {
    pub fn set_elem(elem:&Element) {
        let bounds = elem.get_bounding_client_rect();
        let width = bounds.width() as i32;
        let height = bounds.height() as i32;
        let x = bounds.x() as i32;
        let y = bounds.y() as i32;

        Self::set_direct(width, height, x, y);
    }
    pub fn set_direct(container_width: i32, container_height:i32, container_x: i32, container_y: i32) {
        CONTAINER_WIDTH.store(container_width, Ordering::SeqCst);
        CONTAINER_HEIGHT.store(container_height, Ordering::SeqCst);
        CONTAINER_X.store(container_x, Ordering::SeqCst);
        CONTAINER_Y.store(container_y, Ordering::SeqCst);
        Self::apply_css();
    }

    pub fn get_element_pos_rem(element:&Element) -> (f64, f64) {
        let (x, y) = Self::try_get_element_pos(element).unwrap_throw();
        //10.0 gets us from px to rem - but also need to account for scale
        let scale = 10.0 * Self::scale();
        (x / scale, y / scale)
    }

    pub fn get_element_pos_px(element:&Element) -> (f64, f64) {
        Self::try_get_element_pos(element).unwrap_throw()
    }

    pub fn try_get_element_pos(element:&Element) -> Option<(f64, f64)> {
        if !Self::valid_size() {
            None
        } else {
            //element rect is in viewport space
            let rect = element.get_bounding_client_rect();
            let elem_x = rect.left();
            let elem_y = rect.top();
           
            //need to offset it by content space
            let x = elem_x - (Self::x() + Self::content_x());
            let y = elem_y - (Self::y() + Self::content_y());

            Some((x, y))
        }
    }

    pub fn valid_size() -> bool {
        if Self::container_width() > 1.0 && Self::container_height() > 1.0 {
            true 
        } else {
            false 
        }
    }

    pub fn container_width() -> f64 {
        CONTAINER_WIDTH.load(Ordering::SeqCst) as f64
    }

    pub fn container_height() -> f64 {
        CONTAINER_HEIGHT.load(Ordering::SeqCst) as f64
    }
    pub fn container_x() -> f64 {
        CONTAINER_X.load(Ordering::SeqCst) as f64
    }

    pub fn container_y() -> f64 {
        CONTAINER_Y.load(Ordering::SeqCst) as f64
    }

    pub fn container_ratio() -> f64 {
        Self::container_width() / Self::container_height()
    }

    pub fn x() -> f64 {
        Self::container_x() + ((Self::container_width() - Self::width()) / 2.0)
    }
    pub fn y() -> f64 {
        Self::container_y() + ((Self::container_height() - Self::height()) / 2.0)
    }

    pub fn width() -> f64 {
        if Self::container_ratio() > STAGE_RATIO {
            Self::container_height() * STAGE_RATIO
        } else {
            Self::container_width()
        }
    }
    pub fn height() -> f64 {
        if Self::container_ratio() < STAGE_RATIO {
            Self::container_width() / STAGE_RATIO
        } else {
            Self::container_height()
        }
    }

    pub fn scale() -> f64 {
        Self::width() / STAGE_WIDTH
    }

    pub fn content_x() -> f64 {
        (STAGE_PADDING_X_PERC/2.0) * Self::width()
    }
    pub fn content_y() -> f64 {
        (STAGE_PADDING_Y_PERC/2.0) * Self::height()
    }
    pub fn content_width() -> f64 {
        let width = Self::width();
        width - (STAGE_PADDING_X_PERC * width)
    }
    pub fn content_height() -> f64 {
        let height = Self::height();
        height - (STAGE_PADDING_Y_PERC * height)
    }

    fn apply_css() {
        if !Self::valid_size() {
            log::warn!("couldn't set CSS since module bounds has no valid size");
            return;
        }
        let style:CssStyleDeclaration = web_sys::window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .document_element()
            .unwrap_throw()
            .unchecked_into::<HtmlElement>()
            .style();


        style.set_property("font-size", &format!("calc(10px * {})", Self::scale()));
        style.set_property("--scale", &format!("{}", Self::scale()));
        style.set_property("--x", &format!("{}px", Self::x()));
        style.set_property("--y", &format!("{}px", Self::y()));
        style.set_property("--width", &format!("{}px", Self::width()));
        style.set_property("--height", &format!("{}px", Self::height()));
        style.set_property("--content-x", &format!("{}px", Self::content_x()));
        style.set_property("--content-y", &format!("{}px", Self::content_y()));
        style.set_property("--content-width", &format!("{}px", Self::content_width()));
        style.set_property("--content-height", &format!("{}px", Self::content_height()));
    }
}


