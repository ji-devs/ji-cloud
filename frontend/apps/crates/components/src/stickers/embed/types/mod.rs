use shared::domain::module::body::{
    Transform,
    _groups::design::{Embed, EmbedHost as RawEmbedHost},
};
use std::rc::Rc;

pub use self::{
    edpuzzle::{EdpuzzleEmbed, PartialEdpuzzleEmbed},
    google_doc::{GoogleDocsEmbed, PartialGoogleDocsEmbed},
    google_form::{GoogleFormsEmbed, PartialGoogleFormsEmbed},
    google_sheet::{GoogleSheetsEmbed, PartialGoogleSheetsEmbed},
    google_slide::{GoogleSlidesEmbed, PartialGoogleSlidesEmbed},
    puzzel::{PartialPuzzelEmbed, PuzzelEmbed},
    quizlet::{PartialQuizletEmbed, QuizletEmbed},
    sutori::{PartialSutoriEmbed, SutoriEmbed},
    thinglink::{PartialThinglinkEmbed, ThinglinkEmbed},
    vimeo::{PartialVimeoEmbed, VimeoEmbed},
    youtube::{PartialYoutubeEmbed, YoutubeEmbed},
};

mod edpuzzle;
mod google_doc;
mod google_form;
mod google_sheet;
mod google_slide;
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
    GoogleDoc(Rc<PartialGoogleDocsEmbed>),
    GoogleForm(Rc<PartialGoogleFormsEmbed>),
    GoogleSheet(Rc<PartialGoogleSheetsEmbed>),
    GoogleSlide(Rc<PartialGoogleSlidesEmbed>),
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
            PartialEmbedHost::GoogleDoc(google_doc) => {
                EmbedHost::GoogleDoc(Rc::new(google_doc.full()?))
            }
            PartialEmbedHost::GoogleForm(google_form) => {
                EmbedHost::GoogleForm(Rc::new(google_form.full()?))
            }
            PartialEmbedHost::GoogleSheet(google_sheet) => {
                EmbedHost::GoogleSheet(Rc::new(google_sheet.full()?))
            }
            PartialEmbedHost::GoogleSlide(google_slide) => {
                EmbedHost::GoogleSlide(Rc::new(google_slide.full()?))
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
    GoogleDoc(Rc<GoogleDocsEmbed>),
    GoogleForm(Rc<GoogleFormsEmbed>),
    GoogleSheet(Rc<GoogleSheetsEmbed>),
    GoogleSlide(Rc<GoogleSlidesEmbed>),
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
            EmbedHost::GoogleDoc(google_doc) => {
                PartialEmbedHost::GoogleDoc(Rc::new(google_doc.partial()))
            }
            EmbedHost::GoogleForm(google_form) => {
                PartialEmbedHost::GoogleForm(Rc::new(google_form.partial()))
            }
            EmbedHost::GoogleSheet(google_sheet) => {
                PartialEmbedHost::GoogleSheet(Rc::new(google_sheet.partial()))
            }
            EmbedHost::GoogleSlide(google_slide) => {
                PartialEmbedHost::GoogleSlide(Rc::new(google_slide.partial()))
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
            RawEmbedHost::GoogleDoc(google_doc) => Self::GoogleDoc(Rc::new(google_doc.into())),
            RawEmbedHost::GoogleForm(google_form) => Self::GoogleForm(Rc::new(google_form.into())),
            RawEmbedHost::GoogleSheet(google_sheet) => {
                Self::GoogleSheet(Rc::new(google_sheet.into()))
            }
            RawEmbedHost::GoogleSlide(google_slide) => {
                Self::GoogleSlide(Rc::new(google_slide.into()))
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
            EmbedHost::GoogleDoc(google_doc) => RawEmbedHost::GoogleDoc((&**google_doc).into()),
            EmbedHost::GoogleForm(google_form) => RawEmbedHost::GoogleForm((&**google_form).into()),
            EmbedHost::GoogleSheet(google_sheet) => {
                RawEmbedHost::GoogleSheet((&**google_sheet).into())
            }
            EmbedHost::GoogleSlide(google_slide) => {
                RawEmbedHost::GoogleSlide((&**google_slide).into())
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
