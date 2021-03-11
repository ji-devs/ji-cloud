use std::rc::Rc;
use chrono::Utc;
use itertools::Itertools;

use super::state::{State, UserColor, Color};

pub fn add_user_color(state: Rc<State>, color: Color) {
    let mut user_colors = state.user_colors.lock_mut();
    let first_empty = user_colors.iter().find_position(|c| c.is_none());

    match first_empty {
        Some((empty_slot, _)) => {
            user_colors.set_cloned(empty_slot, Rc::new(Some(UserColor {
                color,
                time_created: Utc::now(),
            })))
        },
        None => {
            let (oldest, _) = user_colors.iter()
                .enumerate()
                .map(|(index, element)| (index, element.as_ref().clone().unwrap().time_created))
                .min_by(|(_, x), (_, y) | x.cmp(y))
                .unwrap();

            user_colors.set_cloned(oldest, Rc::new(Some(UserColor {
                color,
                time_created: Utc::now(),
            })));
        },
    }
}
