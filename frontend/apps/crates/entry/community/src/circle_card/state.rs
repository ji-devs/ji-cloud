use shared::domain::circle::Circle;

// Keeping simple. No new method, Config, Rc. Might not be able to keep it this way
pub struct CircleCard<'a> {
    pub circle: &'a Circle,
}
