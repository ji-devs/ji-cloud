use std::rc::Rc;

use futures_signals::signal::Mutable;
use shared::domain::circle::Circle;

use super::super::callbacks::EditCirclesCallbacks;

pub struct EditName {
    circle: Circle,
    pub callbacks: EditCirclesCallbacks,
    pub display_name: Mutable<String>,
}

impl EditName {
    pub fn new(circle: Circle, callbacks: EditCirclesCallbacks) -> Rc<Self> {
        Rc::new(Self {
            callbacks,
            display_name: Mutable::new(circle.display_name.clone()),
            circle,
        })
    }

    pub fn get_circle_update_data(&self) -> Circle {
        let mut circle = self.circle.clone();

        circle.display_name = self.display_name.get_cloned();

        circle
    }
}
