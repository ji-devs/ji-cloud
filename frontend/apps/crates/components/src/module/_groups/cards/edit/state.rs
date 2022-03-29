use std::rc::Rc;

use super::debug::DebugSettings;
use crate::{
    module::{_common::edit::prelude::*, _groups::cards::lookup::Side},
    tooltip::state::State as TooltipState,
};
use dominator_helpers::signals::EitherSignal;
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::jig::{
    module::{
        body::{
            Audio, Background, BodyExt, Image, Instructions,
            _groups::cards::{self as raw, BaseContent, Mode, Step},
        },
        ModuleId,
    },
    JigId, ModuleKind,
};
use utils::prelude::*;

pub trait RawDataExt: BodyExt<Mode, Step> + 'static {
    fn get_content(&self) -> Option<&BaseContent>;
    fn get_content_mut(&mut self) -> Option<&mut BaseContent>;
}

impl RawDataExt for shared::domain::jig::module::body::memory::ModuleData {
    fn get_content(&self) -> Option<&BaseContent> {
        self.content.as_ref().map(|content| &content.base)
    }
    fn get_content_mut(&mut self) -> Option<&mut BaseContent> {
        self.content.as_mut().map(|content| &mut content.base)
    }
}
impl RawDataExt for shared::domain::jig::module::body::flashcards::ModuleData {
    fn get_content(&self) -> Option<&BaseContent> {
        self.content.as_ref().map(|content| &content.base)
    }
    fn get_content_mut(&mut self) -> Option<&mut BaseContent> {
        self.content.as_mut().map(|content| &mut content.base)
    }
}
impl RawDataExt for shared::domain::jig::module::body::matching::ModuleData {
    fn get_content(&self) -> Option<&BaseContent> {
        self.content.as_ref().map(|content| &content.base)
    }
    fn get_content_mut(&mut self) -> Option<&mut BaseContent> {
        self.content.as_mut().map(|content| &mut content.base)
    }
}
impl RawDataExt for shared::domain::jig::module::body::card_quiz::ModuleData {
    fn get_content(&self) -> Option<&BaseContent> {
        self.content.as_ref().map(|content| &content.base)
    }
    fn get_content_mut(&mut self) -> Option<&mut BaseContent> {
        self.content.as_mut().map(|content| &mut content.base)
    }
}

pub trait ExtraExt: 'static {}

pub struct CardsBase<RawData: RawDataExt, E: ExtraExt> {
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub mode: Mode,
    pub module_kind: ModuleKind,
    pub tooltips: Tooltips,
    pub pairs: MutableVec<(Card, Card)>,
    pub selected_pair: Mutable<Option<(usize, SelectedSide)>>,
    pub background: Mutable<Option<Background>>,
    pub extra: E,
    pub debug: DebugSettings,
}

#[derive(Clone)]
pub enum SelectedSide {
    One(Side),
    Both,
}

pub struct Tooltips {
    pub delete: Mutable<Option<Rc<TooltipState>>>,
    pub list_error: Mutable<Option<Rc<TooltipState>>>,
}
impl Tooltips {
    pub fn new() -> Self {
        Self {
            delete: Mutable::new(None),
            list_error: Mutable::new(None),
        }
    }
}

impl<RawData: RawDataExt, E: ExtraExt> CardsBase<RawData, E> {
    pub async fn new(
        init_args: BaseInitFromRawArgs<RawData, Mode, Step>,
        extra: E,
        debug: Option<DebugSettings>,
    ) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            jig_id,
            module_id,
            theme_id,
            history,
            step,
            module_kind,
            ..
        } = init_args;

        let content = raw.get_content().unwrap_ji().clone();

        let pairs: Vec<(Card, Card)> = content
            .pairs
            .iter()
            .map(|pair| (pair.0.clone().into(), pair.1.clone().into()))
            .collect();

        let mode = content.mode;
        let instructions = Mutable::new(content.instructions);

        let background = Mutable::new(content.background);

        Rc::new(Self {
            jig_id,
            module_id,
            history,
            step: step.read_only(),
            theme_id,
            instructions,
            mode,
            tooltips: Tooltips::new(),
            pairs: MutableVec::new_with_values(pairs),
            selected_pair: Mutable::new(None),
            background,
            extra,
            module_kind,
            debug: debug.unwrap_or_default(),
        })
    }

    pub fn clone_pairs_raw(&self) -> Vec<(raw::Card, raw::Card)> {
        self.pairs
            .lock_ref()
            .iter()
            .map(|pair| (pair.0.clone().into(), pair.1.clone().into()))
            .collect()
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> {
        self.pairs
            .signal_vec_cloned()
            .len()
            .map(|len| len == 0)
            .dedupe()
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id.signal().map(|id| id.as_str_id())
    }
}

//the requirement for this indirection might be a compiler bug...
//I couldn't reproduce it on playground
//here was the latest attempt: https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=75e158fa8d226b8fdc505ec8551ca259

impl<RawData: RawDataExt, E: ExtraExt> BaseExt<Step> for CardsBase<RawData, E> {
    type NextStepAllowedSignal = impl Signal<Item = bool>;

    fn get_jig_id(&self) -> JigId {
        self.jig_id
    }

    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }

    fn allowed_step_change(&self, _from: Step, _to: Step) -> bool {
        self.pairs
            .lock_ref()
            .iter()
            .filter(|(card_1, card_2)| card_1.get_is_valid_data() && card_2.get_is_valid_data())
            .count()
            >= 2
    }

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal {
        let _mode = self.mode;

        self.pairs
            .signal_vec_cloned()
            .map_signal(|(card_1, card_2)| {
                map_ref! {
                    let card_1_valid = card_1.is_valid_data_signal(),
                    let card_2_valid = card_2.is_valid_data_signal()
                        => {
                            *card_1_valid && *card_2_valid
                        }
                }
            })
            .to_signal_map(|xs| xs.iter().filter(|x| **x).count() >= 2)
    }
}

#[derive(Clone)]
pub struct Card {
    pub audio: Option<Audio>,
    pub card_content: CardContent,
}

#[derive(Clone)]
pub enum CardContent {
    Text(Mutable<String>),
    Image(Mutable<Option<Image>>),
}

impl Card {
    pub fn new_text(data: String) -> Self {
        Self {
            audio: None,
            card_content: CardContent::Text(Mutable::new(data)),
        }
    }
    pub fn new_image(data: Option<Image>) -> Self {
        Self {
            audio: None,
            card_content: CardContent::Image(Mutable::new(data)),
        }
    }

    pub fn as_text_mutable(&self) -> &Mutable<String> {
        match &self.card_content {
            CardContent::Text(m) => m,
            _ => panic!("not a text type!"),
        }
    }
    pub fn as_image_mutable(&self) -> &Mutable<Option<Image>> {
        match &self.card_content {
            CardContent::Image(m) => m,
            _ => panic!("not an image type!"),
        }
    }

    pub fn get_is_valid_data(&self) -> bool {
        match &self.card_content {
            CardContent::Text(text) => !text.lock_ref().is_empty(),
            CardContent::Image(image) => image.lock_ref().is_some(),
        }
    }

    pub fn is_valid_data_signal(&self) -> impl Signal<Item = bool> {
        match &self.card_content {
            CardContent::Text(text) => EitherSignal::Left(text.signal_ref(|text| !text.is_empty())),
            CardContent::Image(image) => {
                EitherSignal::Right(image.signal_ref(|image| image.is_some()))
            }
        }
    }
}

impl From<raw::Card> for Card {
    fn from(raw_card: raw::Card) -> Self {
        let card_content = match raw_card.card_content {
            raw::CardContent::Text(x) => CardContent::Text(Mutable::new(x)),
            raw::CardContent::Image(x) => CardContent::Image(Mutable::new(x)),
        };

        Self {
            audio: raw_card.audio,
            card_content,
        }
    }
}

impl From<Card> for raw::Card {
    fn from(card: Card) -> Self {
        let card_content = match card.card_content {
            CardContent::Text(x) => raw::CardContent::Text(x.get_cloned()),
            CardContent::Image(x) => raw::CardContent::Image(x.get_cloned()),
        };

        Self {
            audio: card.audio,
            card_content,
        }
    }
}
