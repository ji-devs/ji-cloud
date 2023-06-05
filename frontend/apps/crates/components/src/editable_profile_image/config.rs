use shared::domain::image::ImageId;

pub struct EditableProfileImageConfig {
    pub save_changes: Box<dyn Fn(Option<ImageId>)>,
}

impl Default for EditableProfileImageConfig {
    fn default() -> Self {
        Self {
            save_changes: Box::new(|_| {}),
        }
    }
}
