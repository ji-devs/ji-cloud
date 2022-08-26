use shared::domain::circle::Circle;

pub struct EditCirclesCallbacks {
    pub save_changes: Box<dyn Fn(Circle)>,
    pub close: Box<dyn Fn()>,
}
