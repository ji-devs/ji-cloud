use super::MaxSize;
use web_sys::File;

pub const STR_WRONG_FILE_TYPE: &str = "Oh no! We don't accept this type of file.";
pub const STR_FILE_TOO_LARGE: &str = "Oh no! This file is too heavy. Maximum file size: ";

pub struct FileInputConfig {
    pub value: Option<File>,
    pub on_change: Box<dyn Fn(Option<File>)>,
    pub max_size: MaxSize,
    pub accept: &'static str,
    pub slot: Option<&'static str>,
    pub show_border: bool,
    pub preview_images: bool,
    pub error_msg_type: String,
    pub error_msg_size: String,
}

impl Default for FileInputConfig {
    fn default() -> Self {
        Self {
            error_msg_type: STR_WRONG_FILE_TYPE.to_string(),
            error_msg_size: format!("{}{}", STR_FILE_TOO_LARGE, MaxSize::default().to_string()),
            show_border: true,
            on_change: Box::new(|_| {}),
            value: Default::default(),
            max_size: Default::default(),
            accept: Default::default(),
            slot: Default::default(),
            preview_images: Default::default(),
        }
    }
}
