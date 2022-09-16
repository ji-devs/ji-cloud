use super::MaxSize;
use web_sys::File;

pub struct FileInputConfig {
    pub value: Option<File>,
    pub on_change: Box<dyn Fn(Option<File>)>,
    pub max_size: MaxSize,
    pub accept: &'static str,
    pub slot: Option<&'static str,>,
    pub show_border: bool,
}

impl Default for FileInputConfig {
    fn default() -> Self {
        Self {
            show_border: true,
            on_change: Box::new(|_| {}),
            value: Default::default(),
            max_size: Default::default(),
            accept: Default::default(),
            slot: Default::default(),
        }
    }
}
