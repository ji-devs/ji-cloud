use super::state::*;
use utils::prelude::*;
use zxcvbn::zxcvbn;

impl PasswordState {
    pub fn update_strength(&self) {
        let password = &self.value.borrow();
        if crate::debug::settings().skip_password_strength {
            self.strength.set(PasswordStrength::Strong);
        } else if password.is_empty() {
            self.strength.set(PasswordStrength::None);
        } else {
            let estimate = zxcvbn(password, &[]).unwrap_ji();
            self.strength.set(estimate.into());
        }
    }
}
