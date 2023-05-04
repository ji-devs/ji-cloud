use shared::domain::module::body::{
    Transform,
    _groups::design::{Embed, EmbedHost as RawEmbedHost},
};
use std::rc::Rc;

pub use self::{
    edpuzzle::{EdpuzzleEmbed, PartialEdpuzzleEmbed},
    google_sheet::{GoogleSheetsEmbed, PartialGoogleSheetsEmbed},
    puzzel::{PartialPuzzelEmbed, PuzzelEmbed},
    quizlet::{PartialQuizletEmbed, QuizletEmbed},
    sutori::{PartialSutoriEmbed, SutoriEmbed},
    thinglink::{PartialThinglinkEmbed, ThinglinkEmbed},
    vimeo::{PartialVimeoEmbed, VimeoEmbed},
    youtube::{PartialYoutubeEmbed, YoutubeEmbed},
};

mod edpuzzle;
mod google_sheet;
mod puzzel;
mod quizlet;
mod sutori;
mod thinglink;
mod vimeo;
mod youtube;

// partial host are host that don't have all required fields, e.g. youtube.url
#[derive(Clone, Debug)]
pub enum PartialEmbedHost {
    Youtube(Rc<PartialYoutubeEmbed>),
    Vimeo(Rc<PartialVimeoEmbed>),
    GoogleSheet(Rc<PartialGoogleSheetsEmbed>),
    Edpuzzle(Rc<PartialEdpuzzleEmbed>),
    Puzzel(Rc<PartialPuzzelEmbed>),
    Quizlet(Rc<PartialQuizletEmbed>),
    Thinglink(Rc<PartialThinglinkEmbed>),
    Sutori(Rc<PartialSutoriEmbed>),
}
impl PartialEmbedHost {
    pub fn full(&self) -> anyhow::Result<EmbedHost> {
        Ok(match self {
            PartialEmbedHost::Youtube(youtube) => EmbedHost::Youtube(Rc::new(youtube.full()?)),
            PartialEmbedHost::Vimeo(vimeo) => EmbedHost::Vimeo(Rc::new(vimeo.full()?)),
            PartialEmbedHost::GoogleSheet(google_sheet) => {
                EmbedHost::GoogleSheet(Rc::new(google_sheet.full()?))
            }
            PartialEmbedHost::Edpuzzle(edpuzzle) => EmbedHost::Edpuzzle(Rc::new(edpuzzle.full()?)),
            PartialEmbedHost::Puzzel(puzzel) => EmbedHost::Puzzel(Rc::new(puzzel.full()?)),
            PartialEmbedHost::Quizlet(quizlet) => EmbedHost::Quizlet(Rc::new(quizlet.full()?)),
            PartialEmbedHost::Thinglink(thinglink) => {
                EmbedHost::Thinglink(Rc::new(thinglink.full()?))
            }
            PartialEmbedHost::Sutori(sutori) => EmbedHost::Sutori(Rc::new(sutori.full()?)),
        })
    }
}
#[derive(Clone, Debug)]
pub enum EmbedHost {
    Youtube(Rc<YoutubeEmbed>),
    Vimeo(Rc<VimeoEmbed>),
    GoogleSheet(Rc<GoogleSheetsEmbed>),
    Edpuzzle(Rc<EdpuzzleEmbed>),
    Puzzel(Rc<PuzzelEmbed>),
    Quizlet(Rc<QuizletEmbed>),
    Thinglink(Rc<ThinglinkEmbed>),
    Sutori(Rc<SutoriEmbed>),
}
impl EmbedHost {
    pub fn partial(&self) -> PartialEmbedHost {
        match self {
            EmbedHost::Youtube(youtube) => PartialEmbedHost::Youtube(Rc::new(youtube.partial())),
            EmbedHost::Vimeo(vimeo) => PartialEmbedHost::Vimeo(Rc::new(vimeo.partial())),
            EmbedHost::GoogleSheet(google_sheet) => {
                PartialEmbedHost::GoogleSheet(Rc::new(google_sheet.partial()))
            }
            EmbedHost::Edpuzzle(edpuzzle) => {
                PartialEmbedHost::Edpuzzle(Rc::new(edpuzzle.partial()))
            }
            EmbedHost::Puzzel(puzzel) => PartialEmbedHost::Puzzel(Rc::new(puzzel.partial())),
            EmbedHost::Quizlet(quizlet) => PartialEmbedHost::Quizlet(Rc::new(quizlet.partial())),
            EmbedHost::Thinglink(thinglink) => {
                PartialEmbedHost::Thinglink(Rc::new(thinglink.partial()))
            }
            EmbedHost::Sutori(sutori) => PartialEmbedHost::Sutori(Rc::new(sutori.partial())),
        }
    }
}
impl From<RawEmbedHost> for EmbedHost {
    fn from(value: RawEmbedHost) -> Self {
        match value {
            RawEmbedHost::Youtube(youtube) => Self::Youtube(Rc::new(youtube.into())),
            RawEmbedHost::Vimeo(vimeo) => Self::Vimeo(Rc::new(vimeo.into())),
            RawEmbedHost::GoogleSheet(google_sheet) => {
                Self::GoogleSheet(Rc::new(google_sheet.into()))
            }
            RawEmbedHost::Edpuzzle(edpuzzle) => Self::Edpuzzle(Rc::new(edpuzzle.into())),
            RawEmbedHost::Puzzel(puzzel) => Self::Puzzel(Rc::new(puzzel.into())),
            RawEmbedHost::Quizlet(quizlet) => Self::Quizlet(Rc::new(quizlet.into())),
            RawEmbedHost::Thinglink(thinglink) => Self::Thinglink(Rc::new(thinglink.into())),
            RawEmbedHost::Sutori(sutori) => Self::Sutori(Rc::new(sutori.into())),
        }
    }
}
impl From<&EmbedHost> for RawEmbedHost {
    fn from(value: &EmbedHost) -> Self {
        match value {
            EmbedHost::Youtube(youtube) => RawEmbedHost::Youtube((&**youtube).into()),
            EmbedHost::Vimeo(vimeo) => RawEmbedHost::Vimeo((&**vimeo).into()),
            EmbedHost::GoogleSheet(google_sheet) => {
                RawEmbedHost::GoogleSheet((&**google_sheet).into())
            }
            EmbedHost::Edpuzzle(edpuzzle) => RawEmbedHost::Edpuzzle((&**edpuzzle).into()),
            EmbedHost::Puzzel(puzzel) => RawEmbedHost::Puzzel((&**puzzel).into()),
            EmbedHost::Quizlet(quizlet) => RawEmbedHost::Quizlet((&**quizlet).into()),
            EmbedHost::Thinglink(thinglink) => RawEmbedHost::Thinglink((&**thinglink).into()),
            EmbedHost::Sutori(sutori) => RawEmbedHost::Sutori((&**sutori).into()),
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
