use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CssStyleDeclaration, HtmlElement};
use config::{STAGE_WIDTH, STAGE_HEIGHT, STAGE_PADDING_X_PERC, STAGE_PADDING_Y_PERC, STAGE_RATIO};

pub struct ModuleBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64
}

impl ModuleBounds {
    pub fn set(container_width: f64, container_height:f64) {
        if let Some(bounds) = Self::try_new(container_width, container_height) {
            bounds.apply_css();
        } else {
            log::warn!("container not ready for resize");
        }
    }

    fn try_new(container_width: f64, container_height: f64) -> Option<Self> {
        if container_width < 1.0 || container_height < 1.0 {
            return None;
        }
       
        let mut width = container_width;
        let mut height = container_height;
        let container_ratio = container_width / container_height;

        if(container_ratio > STAGE_RATIO) {
            width = height * STAGE_RATIO;
        } else {
            height = width / STAGE_RATIO;
        }

        let x = (container_width - width) / 2.0;
        let y = (container_height - height) / 2.0;

        let scale = width / STAGE_WIDTH;

        Some(Self {
            x,
            y,
            width,
            height,
            scale
        })
    }
    fn apply_css(&self) {
        let style:CssStyleDeclaration = web_sys::window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .document_element()
            .unwrap_throw()
            .unchecked_into::<HtmlElement>()
            .style();

            let Self {x, y, width, height, scale} = self;
            
            let content_x = (STAGE_PADDING_X_PERC/2.0) * width;
            let content_y = (STAGE_PADDING_Y_PERC/2.0) * height;
            let content_width = width - (STAGE_PADDING_X_PERC * width);
            let content_height = height - (STAGE_PADDING_Y_PERC * height);

            style.set_property("font-size", &format!("calc(10px * {})", scale));
            style.set_property("--scale", &format!("{}", scale));
            style.set_property("--x", &format!("{}px", x));
            style.set_property("--y", &format!("{}px", y));
            style.set_property("--width", &format!("{}px", width));
            style.set_property("--height", &format!("{}px", height));
            style.set_property("--content-x", &format!("{}px", content_x));
            style.set_property("--content-y", &format!("{}px", content_y));
            style.set_property("--content-width", &format!("{}px", content_width));
            style.set_property("--content-height", &format!("{}px", content_height));
    }
}
