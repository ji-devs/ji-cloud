use std::rc::Rc;

use futures_signals::signal::Mutable;
use rgb::RGBA8;
use strum_macros::{EnumIter, Display};


#[derive(Clone, EnumIter, Display, PartialEq)]
pub enum Font {
    Arial,
    Roboto,
    #[strum(serialize = "Open Sans")]
    OpenSans,
}

#[derive(Clone, EnumIter, Display, PartialEq)]
pub enum ElementType {
    H1,
    H2,
    P1,
    P2,
}

#[derive(Clone, EnumIter, Display, PartialEq)]
pub enum Weight {
    Bolder,
    Bold,
    Normal,
    Lighter,
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

pub type FontSize = u8;
pub type IndentCount = u8;

pub struct State {
    pub font: Mutable<Font>,
    pub element: Mutable<ElementType>,
    pub weight: Mutable<Weight>,
    pub align: Mutable<Align>,
    pub font_size: Mutable<FontSize>,
    pub color: Mutable<Option<RGBA8>>,
    pub highlight_color: Mutable<Option<RGBA8>>,
    // pub indent_count: Mutable<IndentCount>, this will depent on the wysiwyg
    pub bold: Mutable<bool>,
    pub italic: Mutable<bool>,
    pub underline: Mutable<bool>,

    
}

impl State {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            font: Mutable::new(Font::Arial),
            element: Mutable::new(ElementType::H1),
            weight: Mutable::new(Weight::Normal),
            align: Mutable::new(Align::Left),
            font_size: Mutable::new(24),
            color: Mutable::new(None),
            highlight_color: Mutable::new(None),
            // indent_count: Mutable::new(0),
            bold: Mutable::new(false),
            italic: Mutable::new(false),
            underline: Mutable::new(false),
        })
    }
}
