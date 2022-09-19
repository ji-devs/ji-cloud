use std::{fmt::Display, rc::Rc};

use futures_signals::{
    map_ref,
    signal::{Mutable, Signal},
};
use web_sys::File;

mod actions;
mod config;
mod dom;

pub use config::*;

pub struct FileInput {
    value: Mutable<Option<File>>,
    on_change: Box<dyn Fn(Option<File>)>,
    max_size: MaxSize,
    error_size: Mutable<bool>,
    error_mime_type: Mutable<bool>,
    accept: &'static str,
    slot: Option<&'static str>,
    show_border: bool,
    preview_images: bool,
}

impl FileInput {
    pub fn new(config: FileInputConfig) -> Rc<Self> {
        Rc::new(Self {
            value: Mutable::new(config.value),
            on_change: config.on_change,
            max_size: config.max_size,
            error_size: Mutable::new(false),
            error_mime_type: Mutable::new(false),
            accept: config.accept,
            slot: config.slot,
            show_border: config.show_border,
            preview_images: config.preview_images,
        })
    }

    fn has_error_signal(&self) -> impl Signal<Item = bool> {
        map_ref! {
            let error_size = self.error_size.signal(),
            let error_mime_type = self.error_mime_type.signal() => {
                *error_size || *error_mime_type
            }
        }
    }
}

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum MaxSize {
    #[default]
    MB5 = 1024 * 1024 * 5,
}

impl Display for MaxSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MaxSize::MB5 => "5 MB",
        };
        write!(f, "{}", s)
    }
}
