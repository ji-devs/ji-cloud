use anyhow::Context;
use futures_signals::signal::Mutable;
use shared::domain::module::body::_groups::design::{
    DoneAction, EmbedHost as RawEmbedHost, YoutubeEmbed as RawYoutubeEmbed, YoutubeUrl,
};
use std::rc::Rc;

// partial host are host that don't have all required fields, e.g. youtube.url
#[derive(Clone, Debug)]
pub enum PartialEmbedHost {
    Youtube(Rc<PartialYoutubeEmbed>),
    // GoogleSheet(Rc<PartialGoogleSheetsEmbed>),
}
impl PartialEmbedHost {
    pub fn full(&self) -> anyhow::Result<EmbedHost> {
        Ok(match self {
            PartialEmbedHost::Youtube(youtube) => EmbedHost::Youtube(Rc::new(youtube.full()?)),
        })
    }
}
#[derive(Clone, Debug)]
pub enum EmbedHost {
    Youtube(Rc<YoutubeEmbed>),
    // GoogleSheet(Rc<GoogleSheetsEmbed>),
}
impl EmbedHost {
    pub fn partial(&self) -> PartialEmbedHost {
        match self {
            EmbedHost::Youtube(youtube) => PartialEmbedHost::Youtube(Rc::new(youtube.partial())),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialEmbedHost) -> anyhow::Result<()> {
        match (self, partial) {
            (EmbedHost::Youtube(youtube), PartialEmbedHost::Youtube(partial)) => {
                youtube.update_from_partial(&*partial)
            },
            // _ => panic!() TODO: enable once there's more then open enum variant
        }
    }
}
impl From<RawEmbedHost> for EmbedHost {
    fn from(value: RawEmbedHost) -> Self {
        match value {
            RawEmbedHost::Youtube(youtube) => Self::Youtube(Rc::new(youtube.into())),
            // RawEmbedHost::GoogleSheet(google_sheet) => Self::GoogleSheet(Rc::new(google_sheet.into())),
        }
    }
}
impl From<&EmbedHost> for RawEmbedHost {
    fn from(value: &EmbedHost) -> Self {
        match value {
            EmbedHost::Youtube(youtube) => RawEmbedHost::Youtube((&**youtube).into()),
            // EmbedHost::GoogleSheet(google_sheet) => RawEmbedHost::GoogleSheet((&*google_sheet).try_into()?),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PartialYoutubeEmbed {
    pub url: Mutable<Option<YoutubeUrl>>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl PartialYoutubeEmbed {
    pub fn full(&self) -> anyhow::Result<YoutubeEmbed> {
        Ok(YoutubeEmbed {
            url: Mutable::new(self.url.get_cloned().context("")?),
            clip: self.clip.clone(),
            start_at: self.start_at.clone(),
            end_at: self.end_at.clone(),
            captions: self.captions.clone(),
            muted: self.muted.clone(),
            autoplay: self.autoplay.clone(),
            done_action: self.done_action.clone(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct YoutubeEmbed {
    pub url: Mutable<YoutubeUrl>,
    pub clip: Mutable<bool>,
    pub start_at: Mutable<Option<u32>>,
    pub end_at: Mutable<Option<u32>>,
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}
impl YoutubeEmbed {
    pub fn partial(&self) -> PartialYoutubeEmbed {
        PartialYoutubeEmbed {
            url: Mutable::new(Some(self.url.get_cloned())),
            clip: self.clip.clone(),
            start_at: self.start_at.clone(),
            end_at: self.end_at.clone(),
            captions: self.captions.clone(),
            muted: self.muted.clone(),
            autoplay: self.autoplay.clone(),
            done_action: self.done_action.clone(),
        }
    }
    pub fn update_from_partial(&self, partial: &PartialYoutubeEmbed) -> anyhow::Result<()> {
        self.clip.set_neq(partial.clip.get());
        self.start_at.set_neq(partial.start_at.get());
        self.end_at.set_neq(partial.end_at.get());
        self.captions.set_neq(partial.captions.get());
        self.muted.set_neq(partial.muted.get());
        self.autoplay.set_neq(partial.autoplay.get());
        self.done_action.set_neq(partial.done_action.get());
        self.url.set_neq(partial.url.get_cloned().context("")?);
        Ok(())
    }
}
impl From<RawYoutubeEmbed> for YoutubeEmbed {
    fn from(value: RawYoutubeEmbed) -> Self {
        Self {
            url: Mutable::new(value.url),
            clip: Mutable::new(value.start_at.is_some() || value.end_at.is_some()),
            start_at: Mutable::new(value.start_at),
            end_at: Mutable::new(value.end_at),
            captions: Mutable::new(value.captions),
            muted: Mutable::new(value.muted),
            autoplay: Mutable::new(value.autoplay),
            done_action: Mutable::new(value.done_action),
        }
    }
}
impl From<&YoutubeEmbed> for RawYoutubeEmbed {
    fn from(value: &YoutubeEmbed) -> Self {
        Self {
            url: value.url.get_cloned(),
            start_at: value.start_at.get(),
            end_at: value.end_at.get(),
            captions: value.captions.get(),
            muted: value.muted.get(),
            autoplay: value.autoplay.get(),
            done_action: value.done_action.get(),
        }
    }
}

// #[derive(Clone, Default)]
// pub struct GoogleSheetsEmbed {
//     pub url: Mutable<Option<GoogleSheetId>>,
// }
// impl From<RawGoogleSheetsEmbed> for GoogleSheetsEmbed {
//     fn from(value: RawGoogleSheetsEmbed) -> Self {
//         Self {
//             url: Mutable::new(Some(value.url)),
//         }
//     }
// }
// impl TryFrom<&GoogleSheetsEmbed> for RawGoogleSheetsEmbed {
//     type Error = anyhow::Error;
//     fn try_from(value: &GoogleSheetsEmbed) -> Result<Self, Self::Error> {
//         Ok(Self {
//             url: value.url.get_cloned()?,
//         })
//     }
// }
