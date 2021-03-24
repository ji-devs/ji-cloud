use super::state::*;
use gloo_timers::future::TimeoutFuture;

impl State {
    pub async fn evaluate(&self, id_1: usize, id_2: usize) {

        /*
        if self.pair_lookup[id_1] == id_2 {
            let main_cards = self.main_cards.lock_ref();
            let mut found_pairs = self.found_pairs.borrow_mut();
            let found_pairs_index = found_pairs.len();
            found_pairs.push((id_1, id_2));
            if let Some(card) = main_cards.iter().find(|c| c.id == id_1) {
                card.found.set(Some(found_pairs_index));
            }
            if let Some(card) = main_cards.iter().find(|c| c.id == id_2) {
                card.found.set(Some(found_pairs_index));
            }
        } else {
            TimeoutFuture::new(2_000).await;
        }
        self.flip_state.set(FlipState::None);
        */
    }
    pub fn grid_number(&self) -> usize { 
        let n_cards = self.cards.len();
        match n_cards {
            8 => 1,
            10 => 2,
            12 => 1,
            14 => 5,
            16 => 1,
            18 => 6,
            20 => 2,
            22 => 7,
            24 => 3,
            26 => 8,
            28 => 4,
            _ => panic!("no known grid number for {} cards!", n_cards)
        }
    }

    pub fn card_click(&self, id: usize) {
        let flip_state = &mut *self.flip_state.lock_mut();

        match flip_state {
            FlipState::None => *flip_state = FlipState::One(id),
            FlipState::One(other) => {
                if *other != id {
                    *flip_state = FlipState::Two((id, *other))
                }
            },
            _ => {}
        }
    }
}
