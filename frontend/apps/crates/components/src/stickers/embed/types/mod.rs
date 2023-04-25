use shared::domain::module::body::{
    Transform,
    _groups::design::{Embed, EmbedHost as RawEmbedHost},
};
use std::rc::Rc;

mod google_sheet;
mod youtube;

pub use google_sheet::*;
pub use youtube::*;

// partial host are host that don't have all required fields, e.g. youtube.url
#[derive(Clone, Debug)]
pub enum PartialEmbedHost {
    Youtube(Rc<PartialYoutubeEmbed>),
    GoogleSheet(Rc<PartialGoogleSheetsEmbed>),
}
impl PartialEmbedHost {
    pub fn full(&self) -> anyhow::Result<EmbedHost> {
        Ok(match self {
            PartialEmbedHost::Youtube(youtube) => EmbedHost::Youtube(Rc::new(youtube.full()?)),
            PartialEmbedHost::GoogleSheet(google_sheet) => {
                EmbedHost::GoogleSheet(Rc::new(google_sheet.full()?))
            }
        })
    }
}
#[derive(Clone, Debug)]
pub enum EmbedHost {
    Youtube(Rc<YoutubeEmbed>),
    GoogleSheet(Rc<GoogleSheetsEmbed>),
}
impl EmbedHost {
    pub fn partial(&self) -> PartialEmbedHost {
        match self {
            EmbedHost::Youtube(youtube) => PartialEmbedHost::Youtube(Rc::new(youtube.partial())),
            EmbedHost::GoogleSheet(google_sheet) => {
                PartialEmbedHost::GoogleSheet(Rc::new(google_sheet.partial()))
            }
        }
    }
    pub fn update_from_partial(&self, partial: &PartialEmbedHost) -> anyhow::Result<()> {
        match (self, partial) {
            (EmbedHost::Youtube(youtube), PartialEmbedHost::Youtube(partial)) => {
                youtube.update_from_partial(&*partial)
            }
            (EmbedHost::GoogleSheet(google_sheet), PartialEmbedHost::GoogleSheet(partial)) => {
                google_sheet.update_from_partial(&*partial)
            }
            _ => panic!(),
        }
    }
}
impl From<RawEmbedHost> for EmbedHost {
    fn from(value: RawEmbedHost) -> Self {
        match value {
            RawEmbedHost::Youtube(youtube) => Self::Youtube(Rc::new(youtube.into())),
            RawEmbedHost::GoogleSheet(google_sheet) => {
                Self::GoogleSheet(Rc::new(google_sheet.into()))
            }
        }
    }
}
impl From<&EmbedHost> for RawEmbedHost {
    fn from(value: &EmbedHost) -> Self {
        match value {
            EmbedHost::Youtube(youtube) => RawEmbedHost::Youtube((&**youtube).into()),
            EmbedHost::GoogleSheet(google_sheet) => {
                RawEmbedHost::GoogleSheet((&**google_sheet).into())
            }
        }
    }
}

pub trait EmbedExt {
    fn new(value: RawEmbedHost) -> Self;
}

impl EmbedExt for Embed {
    /// Create a new Embed
    fn new(host: RawEmbedHost) -> Self {
        Self {
            host,
            transform: Transform::identity(),
        }
    }
}

pub trait ParseUrlExt: Sized {
    fn try_parse(text: String) -> anyhow::Result<Self>;
    fn get_id(&self) -> &str;
}
