use super::state::Hotspot;
use dominator::animation::Percentage;

impl Hotspot {
    pub fn fade_out(&self) {
        self.fade_animation.animate_to(Percentage::new(1.0));
    }
    pub fn _fade_in(&self) {
        self.fade_animation.animate_to(Percentage::new(0.0));
    }
}
