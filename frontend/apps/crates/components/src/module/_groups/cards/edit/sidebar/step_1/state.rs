use crate::{
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchOptions, State as ImageSearchState},
    },
    lists::{
        dual::{
            callbacks::Callbacks as DualListCallbacks,
            state::{Options as DualListOptions, State as DualListState},
        },
        single::{
            callbacks::Callbacks as SingleListCallbacks,
            state::{Options as SingleListOptions, State as SingleListState},
        },
    },
    module::_groups::cards::edit::{config, state::*, strings},
};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::{Image, _groups::cards::Mode};
use std::rc::Rc;

pub struct Step1<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub widget: Rc<Widget>,
}

impl<RawData: RawDataExt, E: ExtraExt> Step1<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>) -> Rc<Self> {
        let widget = match base.mode {
            Mode::WordsAndImages => {
                let kind = match base.debug.step1_tab {
                    Some(kind) => kind,
                    None => TabKind::Text,
                };

                let tab = Mutable::new(Tab::new(base.clone(), kind));
                Widget::Tabs(tab)
            }
            Mode::Duplicate | Mode::Lettering => {
                Widget::Single(Rc::new(make_single_list(base.clone())))
            }
            _ => Widget::Dual(Rc::new(make_dual_list(base.clone()))),
        };

        Rc::new(Self {
            base,
            widget: Rc::new(widget),
        })
    }
}

#[derive(Clone)]
pub enum Widget {
    Single(Rc<SingleListState>),
    Dual(Rc<DualListState>),
    Tabs(Mutable<Tab>),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TabKind {
    Text,
    Image,
}

impl TabKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Image => "image",
        }
    }
}

#[derive(Clone)]
pub enum Tab {
    Text(Rc<SingleListState>),
    Image(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new<RawData: RawDataExt, E: ExtraExt>(
        base: Rc<CardsBase<RawData, E>>,
        kind: TabKind,
    ) -> Self {
        match kind {
            TabKind::Image => {
                let opts = ImageSearchOptions {
                    background_only: Some(true),
                    upload: true,
                    filters: true,
                    recent: true,
                };

                let callbacks = ImageSearchCallbacks::new(None::<fn(Image)>);
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            TabKind::Text => Self::Text(Rc::new(make_single_list(base.clone()))),
        }
    }

    pub fn kind(&self) -> TabKind {
        match self {
            Self::Text(_) => TabKind::Text,
            Self::Image(_) => TabKind::Image,
        }
    }
}

fn make_single_list<RawData: RawDataExt, E: ExtraExt>(
    base: Rc<CardsBase<RawData, E>>,
) -> SingleListState {
    let _mode = base.mode;

    let callbacks = SingleListCallbacks::new(
        |text| super::actions::limit_text(config::SINGLE_LIST_CHAR_LIMIT, text),
        clone!(base => move |tooltip| {
            base.tooltips.list_error.set(tooltip);
        }),
        clone!(base => move |list| {
            base.replace_single_list(list);
        }),
        |index| config::get_single_list_init_word(index),
    );

    let options = SingleListOptions {
        max_rows: config::MAX_LIST_WORDS,
        min_valid: 2,
    };

    SingleListState::new(options, callbacks)
}
fn make_dual_list<RawData: RawDataExt, E: ExtraExt>(
    base: Rc<CardsBase<RawData, E>>,
) -> DualListState {
    let mode = base.mode;

    let callbacks = DualListCallbacks::new(
        |text| super::actions::limit_text(config::DUAL_LIST_CHAR_LIMIT, text),
        clone!(base => move |tooltip| {
            base.tooltips.list_error.set(tooltip);
        }),
        clone!(base => move |list| {
            base.replace_dual_list(list);
        }),
        |row, index| config::get_dual_list_init_word(row, index),
        clone!(mode => move |side| {
            strings::STR_HEADER(side, mode).to_string()
        }),
    );

    let options = DualListOptions {
        max_rows: config::MAX_LIST_WORDS,
        cell_rows: {
            match base.mode {
                Mode::Riddles => 2,
                _ => 1,
            }
        },
        min_valid: 2,
    };

    DualListState::new(options, callbacks)
}
