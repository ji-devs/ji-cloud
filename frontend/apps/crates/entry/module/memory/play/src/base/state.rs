use shared::{
    config::MAX_LIST_WORDS,
    domain::{
        asset::AssetId,
        jig::player::{ModuleConfig, Seconds},
        module::{
            body::{
                Background, ModuleAssist,
                _groups::cards::{CardPair as RawCardPair, Mode, Step},
                memory::{ModuleData as RawData, PlayerSettings},
                ModuleAssistType,
            },
            ModuleId, StableModuleId,
        },
    },
};

use super::card::state::*;
use components::module::{_common::play::prelude::*, _groups::cards::lookup::Side};
use futures::future::join_all;
use futures_signals::signal::{self, Mutable, ReadOnlyMutable, Signal, SignalExt};
use gloo_timers::future::TimeoutFuture;
use rand::prelude::*;
use std::future::Future;
use std::rc::Rc;
use utils::prelude::*;

pub struct Base {
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub stable_module_id: StableModuleId,
    pub mode: Mode,
    pub pair_lookup: Vec<usize>,
    pub original_pairs: Vec<RawCardPair>,
    pub cards: Vec<Rc<CardState>>,
    pub theme_id: ThemeId,
    pub background: Option<Background>,
    pub flip_state: Mutable<FlipState>,
    pub found_pairs: Mutable<Vec<(usize, usize)>>,
    pub instructions: ModuleAssist,
    /// Feedback to play when the activity ends
    pub feedback: ModuleAssist,
    pub feedback_signal: Mutable<Option<ModuleAssist>>,
    pub settings: PlayerSettings,
    pub module_phase: Mutable<ModulePlayPhase>,
    pub is_animating: Mutable<usize>,
}

#[derive(Debug, Clone)]
pub enum FlipState {
    None,
    One(usize),
    Two(usize, usize),
}
impl Base {
    pub async fn new(init_args: InitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let InitFromRawArgs {
            asset_id,
            module_id,
            stable_module_id,
            asset: _,
            raw,
            theme_id,
            ..
        } = init_args;

        let mut rng = thread_rng();

        let mut content = raw.content.unwrap_ji();

        let max_cards = content.player_settings.pairs_to_display.map_or_else(
            || {
                let card_count = content.base.pairs.len();
                if card_count > MAX_LIST_WORDS {
                    MAX_LIST_WORDS
                } else {
                    card_count
                }
            },
            |pairs_to_display| {
                if pairs_to_display > MAX_LIST_WORDS as u32 {
                    MAX_LIST_WORDS
                } else {
                    pairs_to_display as usize
                }
            },
        );

        let n_cards = max_cards * 2;
        let mut pair_lookup: Vec<usize> = vec![0; n_cards];
        let mut cards = {
            let pairs = &mut content.base.pairs;
            pairs.shuffle(&mut rng);

            let mut cards: Vec<Rc<CardState>> = Vec::with_capacity(n_cards);
            let mut index: usize = 0;

            for pair in pairs.iter().take(max_cards) {
                let (card_1, card_2) = (&pair.0, &pair.1);

                let id_1 = index;
                let id_2 = index + 1;
                index = id_2 + 1;

                cards.push(Rc::new(CardState::new(
                    card_1.clone(),
                    id_1,
                    id_2,
                    Side::Left,
                )));
                cards.push(Rc::new(CardState::new(
                    card_2.clone(),
                    id_2,
                    id_1,
                    Side::Right,
                )));
            }

            cards
        };

        for card in cards.iter() {
            pair_lookup[card.id] = card.other_id;
        }

        if !crate::debug::settings().no_shuffle {
            cards.shuffle(&mut rng);
        }

        Rc::new(Self {
            asset_id,
            module_id,
            stable_module_id,
            mode: content.base.mode,
            pair_lookup,
            original_pairs: content.base.pairs,
            cards,
            theme_id,
            background: content.base.background,
            flip_state: Mutable::new(FlipState::None),
            found_pairs: Mutable::new(Vec::new()),
            instructions: content.base.instructions,
            feedback: content.base.feedback,
            feedback_signal: Mutable::new(None),
            settings: content.player_settings,
            module_phase: init_args.play_phase,
            is_animating: Mutable::new(0),
        })
    }

    pub fn all_cards_ended_future(&self) -> impl Future<Output = bool> {
        let fut = join_all(
            self.cards
                .iter()
                .map(|card| card.ended_signal().wait_for(true)),
        );

        async move { fut.await.into_iter().all(|ended| ended.unwrap_or(false)) }
    }

    pub fn all_cards_ended_signal(&self) -> impl Signal<Item = bool> {
        signal::from_future(self.all_cards_ended_future())
            .map(|s| s.unwrap_or(false))
            .dedupe()
            .throttle(|| TimeoutFuture::new(1_000))
    }
}

impl BaseExt for Base {
    fn module_id(&self) -> ModuleId {
        self.module_id
    }

    fn stable_module_id(&self) -> StableModuleId {
        self.stable_module_id
    }

    fn get_module_assist(&self) -> Option<ModuleAssist> {
        Some(self.instructions.clone())
    }

    fn get_feedback(&self) -> ReadOnlyMutable<Option<ModuleAssist>> {
        self.feedback_signal.read_only()
    }

    fn handle_module_assist_ended(&self, module_assist_type: ModuleAssistType) {
        if let ModuleAssistType::Feedback = module_assist_type {
            self.set_play_phase(ModulePlayPhase::Ending(Some(ModuleEnding::Next)));
        }
    }

    fn get_module_config(&self) -> ModuleConfig {
        ModuleConfig {
            timer: self.settings.time_limit.map(|t| Seconds(t)),
            ..Default::default()
        }
    }

    fn play_phase(&self) -> Mutable<ModulePlayPhase> {
        self.module_phase.clone()
    }
}
