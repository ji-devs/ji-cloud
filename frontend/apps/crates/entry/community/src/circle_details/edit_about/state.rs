use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::circle::Circle;

use super::super::callbacks::EditCirclesCallbacks;

pub struct EditAbout {
    circle: Circle,
    pub callbacks: EditCirclesCallbacks,
    pub description: Mutable<String>,
}

impl EditAbout {
    pub fn new(circle: Circle, callbacks: EditCirclesCallbacks) -> Rc<Self> {
        Rc::new(Self {
            callbacks,
            description: Mutable::new(circle.description.clone()),
            circle,
        })
    }

    pub fn get_circle_update_data(&self) -> Circle {
        let mut circle = self.circle.clone();

        circle.description = self.description.get_cloned();

        circle
    }
}
