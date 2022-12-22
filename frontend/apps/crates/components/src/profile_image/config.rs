use shared::domain::image::ImageId;

pub struct ProfileImageConfig {
    pub save_changes: Box<dyn Fn(Option<ImageId>)>,
}

impl Default for ProfileImageConfig {
    fn default() -> Self {
        Self {
            save_changes: Box::new(|_| {}),
        }
    }
}
