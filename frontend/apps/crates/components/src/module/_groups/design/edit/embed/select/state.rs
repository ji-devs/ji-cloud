use std::rc::Rc;

use futures_signals::signal::Mutable;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::stickers::{
    embed::{state::Embed, types::PartialEmbedHost},
    state::{Sticker, Stickers},
};

#[derive(Clone, Copy, EnumIter, PartialEq, Eq)]
pub(super) enum EmbedHostType {
    Youtube,
    Vimeo,
    GoogleDoc,
    GoogleForm,
    GoogleSheet,
    GoogleSlide,
    // Edpuzzle,
    // Puzzel,
    Quizlet,
    Thinglink,
    Sutori,
}
impl EmbedHostType {
    pub(super) fn display_name(&self) -> &'static str {
        match self {
            EmbedHostType::Youtube => "YouTube",
            EmbedHostType::Vimeo => "Vimeo",
            EmbedHostType::GoogleDoc => "Google Doc",
            EmbedHostType::GoogleForm => "Google Form",
            EmbedHostType::GoogleSheet => "Google Sheet",
            EmbedHostType::GoogleSlide => "Google Slide",
            // EmbedHostType::Edpuzzle => "Edpuzzle",
            // EmbedHostType::Puzzel => "Puzzel",
            EmbedHostType::Quizlet => "Quizlet",
            EmbedHostType::Thinglink => "Thinglink",
            EmbedHostType::Sutori => "Sutori",
        }
    }
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            EmbedHostType::Youtube => "youtube",
            EmbedHostType::Vimeo => "vimeo",
            EmbedHostType::GoogleDoc => "google-docs",
            EmbedHostType::GoogleForm => "google-forms",
            EmbedHostType::GoogleSheet => "google-sheets",
            EmbedHostType::GoogleSlide => "google-slides",
            // EmbedHostType::Edpuzzle => "edpuzzle",
            // EmbedHostType::Puzzel => "puzzel",
            EmbedHostType::Quizlet => "quizlet",
            EmbedHostType::Thinglink => "thinglink",
            EmbedHostType::Sutori => "sutori",
        }
    }
}

pub struct EmbedSelectList {
    pub(super) modules: Vec<EmbedHostType>,
}
impl EmbedSelectList {
    pub fn all() -> Self {
        Self {
            modules: EmbedHostType::iter().collect(),
        }
    }
    pub fn video_only() -> Self {
        Self {
            modules: vec![EmbedHostType::Youtube, EmbedHostType::Vimeo],
        }
    }
}

pub struct EmbedSelect {
    pub(super) stickers: Rc<Stickers<Sticker>>,
    pub(super) embed: Mutable<Option<Rc<Embed>>>,
    pub(super) host: Mutable<Option<PartialEmbedHost>>,
    pub(super) type_list: EmbedSelectList,
}

impl EmbedSelect {
    pub fn new(
        type_list: EmbedSelectList,
        stickers: &Rc<Stickers<Sticker>>,
        embed: Mutable<Option<Rc<Embed>>>,
    ) -> Rc<Self> {
        let host = embed
            .lock_ref()
            .as_ref()
            .map(|host| host.host.lock_ref().partial());

        Rc::new(Self {
            stickers: Rc::clone(&stickers),
            embed,
            host: Mutable::new(host),
            type_list,
        })
    }
}
