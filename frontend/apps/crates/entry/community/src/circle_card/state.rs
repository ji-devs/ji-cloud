use futures_signals::signal::ReadOnlyMutable;
use shared::domain::circle::Circle;

// Keeping simple. No new method, Config, Rc. Might not be able to keep it this way
pub struct CircleCard<'a> {
    pub circle: &'a Circle,
    pub slot: &'a str,
    pub is_member: ReadOnlyMutable<bool>,
    pub on_member: Box<dyn Fn(bool)>,
}
