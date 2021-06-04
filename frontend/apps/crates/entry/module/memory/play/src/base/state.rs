use shared::domain::jig::{
    JigId, 
    Jig, 
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            memory::{Mode as RawMode, ModuleData as RawData, Content as RawContent, CardPair as RawCardPair}
        }
    }
};
use futures_signals::{
    map_ref,
    signal::{self, Signal, SignalExt, Mutable},
    signal_vec::{SignalVec, SignalVecExt, MutableVec},
};
use std::{
    rc::Rc,
    cell::RefCell
};
use rand::prelude::*;
use components::module::play::state::BaseExt;
use utils::prelude::*;
use components::instructions::player::InstructionsPlayer;
use web_sys::AudioContext;
use super::card::state::*;
use std::future::Future;
use futures::future::join_all;
use gloo_timers::future::TimeoutFuture;

pub struct Base {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub mode: RawMode,
    pub pair_lookup: Vec<usize>,
    pub original_pairs: Vec<RawCardPair>,
    pub cards: Vec<Rc<CardState>>,
    pub theme_id: ThemeId,
    pub flip_state: Mutable<FlipState>,
    pub found_pairs: RefCell<Vec<(usize, usize)>>, 
    pub instructions: InstructionsPlayer,
    pub audio_ctx: AudioContext
}

#[derive(Debug, Clone)]
pub enum FlipState {
    None,
    One(usize),
    Two(usize, usize),
}
impl Base {
    pub async fn new(jig_id: JigId, module_id: ModuleId, jig: Option<Jig>, raw:RawData, ) -> Self {
        log::info!("{:?}", raw);

        let audio_ctx = AudioContext::new().unwrap_ji();

        let raw_content = raw.content.unwrap_ji();

        let theme_id = match raw_content.theme {
            ThemeChoice::Jig => {
                // self.jig.as_ref().unwrap_ji().theme_id.clone()
                log::warn!("waiting on jig settings");
                ThemeId::Chalkboard
            },
            ThemeChoice::Override(theme_id) => theme_id
        };

        let n_cards = raw_content.pairs.len() * 2;
        let mut pair_lookup:Vec<usize> = vec![0;n_cards]; 
        let mut cards = { 
            let pairs = &raw_content.pairs;

            let n_cards = pairs.len() * 2;
            let mut cards:Vec<Rc<CardState>> = Vec::with_capacity(n_cards);
            let mut index:usize = 0;

            for pair in pairs.iter() {
                let (card_1, card_2) = (&pair.0, &pair.1);

                let id_1 = index; 
                let id_2 = index + 1;
                index = id_2 + 1;

                cards.push(Rc::new(CardState::new(card_1.into(), id_1, id_2, Side::Left)));
                cards.push(Rc::new(CardState::new(card_2.into(), id_2, id_1, Side::Right)));
            }

            cards
        };

        for card in cards.iter() {
            pair_lookup[card.id] = card.other_id;
        }

        let mut rng = thread_rng();

        if !crate::debug::settings().no_shuffle {
            cards.shuffle(&mut rng);
        }

        Self {
            jig_id,
            module_id,
            mode: raw_content.mode,
            pair_lookup,
            original_pairs: raw_content.pairs,
            cards,
            theme_id,
            flip_state: Mutable::new(FlipState::None), 
            found_pairs: RefCell::new(Vec::new()),
            instructions: InstructionsPlayer::new(raw_content.instructions), 
            audio_ctx,
        }
    }


    pub fn all_cards_ended_future(&self) -> impl Future<Output = bool> {
        let fut = join_all(
            self.cards
                .iter()
                .map(|card| {
                    card
                        .ended_signal()
                        .wait_for(true)
                })
        );

        async move {
            fut.await.into_iter().all(|ended| ended.unwrap_or(false))
        }
    }

    pub fn all_cards_ended_signal(&self) -> impl Signal<Item = bool> {
        signal::from_future(self.all_cards_ended_future())
            .map(|s| s.unwrap_or(false))
            .dedupe()
            .throttle(|| TimeoutFuture::new(1_000))
    }
}

impl BaseExt for Base {
}
