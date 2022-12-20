use shared::domain::{image::ImageId};

pub const STR_WRONG_FILE_TYPE: &str = "Oh no! We don't accept this type of file.";
pub const STR_FILE_TOO_LARGE: &str = "Oh no! This file is too heavy. Maximum file size: ";

pub struct ProfileImageConfig {
    pub close: Box<dyn Fn()>,
    pub save_changes: Box<dyn Fn(Option<ImageId>)>
}

impl Default for ProfileImageConfig {
    fn default() -> Self {
        Self {
            close: Box::new(|| {}),
            save_changes: Box::new(|_| {}),
        }
    }


}
