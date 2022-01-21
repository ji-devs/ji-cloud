use crate::{
    image::search::{
        callbacks::Callbacks as ImageSearchCallbacks,
        state::{ImageSearchKind, ImageSearchOptions, State as ImageSearchState},
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
    tabs::MenuTabKind,
};
use dominator::clone;
use futures_signals::signal::Mutable;
use once_cell::sync::OnceCell;
use shared::{
    domain::jig::module::body::{Image, _groups::cards::Mode},
    config as shared_config,
};
use utils::unwrap::UnwrapJiExt;
use std::rc::Rc;

pub struct Step1<RawData: RawDataExt, E: ExtraExt> {
    pub base: Rc<CardsBase<RawData, E>>,
    pub widget: OnceCell<Widget>,
    pub tab_index: Mutable<Option<usize>>,
}

impl<RawData: RawDataExt, E: ExtraExt> Step1<RawData, E> {
    pub fn new(base: Rc<CardsBase<RawData, E>>, tab_index: Mutable<Option<usize>>) -> Rc<Self> {
        let state = Rc::new(Self {
            base: base.clone(),
            widget: OnceCell::default(),
            tab_index,
        });

        // Widgets require a reference to the top-level state so that they can have access to any
        // fields they might require in callbacks.
        let widget = match base.mode {
            Mode::WordsAndImages => {
                let kind = match base.debug.step1_tab {
                    Some(kind) => kind,
                    None => MenuTabKind::Text,
                };

                let tab = Mutable::new(Tab::new(state.clone(), kind));
                Widget::Tabs(tab)
            }
            Mode::Duplicate | Mode::Lettering => {
                Widget::Single(Rc::new(make_single_list(state.clone())))
            }
            _ => Widget::Dual(Rc::new(make_dual_list(state.clone()))),
        };

        // `set()` will return an Err if the cell already has a value. However, because we are
        // using this purely to lazily set a widget immediately after initializing the state, the
        // error here will never occur.
        let _ = state.widget.set(widget);

        state
    }
}

#[derive(Clone)]
pub enum Widget {
    Single(Rc<SingleListState>),
    Dual(Rc<DualListState>),
    Tabs(Mutable<Tab>),
}

#[derive(Clone)]
pub enum Tab {
    Text(Rc<SingleListState>),
    Image(Rc<ImageSearchState>),
}

impl Tab {
    pub fn new<RawData: RawDataExt, E: ExtraExt>(
        state: Rc<Step1<RawData, E>>,
        kind: MenuTabKind,
    ) -> Self {
        match kind {
            MenuTabKind::Image => {
                let opts = ImageSearchOptions {
                    kind: ImageSearchKind::Sticker,
                    ..ImageSearchOptions::default()
                };

                let callbacks = ImageSearchCallbacks::new(None::<fn(Image)>);
                let state = ImageSearchState::new(opts, callbacks);

                Self::Image(Rc::new(state))
            }
            MenuTabKind::Text => Self::Text(Rc::new(make_single_list(state))),

            _ => unimplemented!("unsupported tab kind!"),
        }
    }

    pub fn kind(&self) -> MenuTabKind {
        match self {
            Self::Text(_) => MenuTabKind::Text,
            Self::Image(_) => MenuTabKind::Image,
        }
    }
    pub fn as_index(&self) -> usize {
        match self {
            Self::Text(_) => 0,
            Self::Image(_) => 1,
        }
    }
}

fn make_single_list<RawData: RawDataExt, E: ExtraExt>(
    state: Rc<Step1<RawData, E>>,
) -> SingleListState {
    let _mode = state.base.mode;

    let callbacks = SingleListCallbacks::new(
        |text| super::actions::limit_text(config::SINGLE_LIST_CHAR_LIMIT, text),
        clone!(state => move |tooltip| {
            state.base.tooltips.list_error.set(tooltip);
        }),
        clone!(state => move |list| {
            state.base.replace_single_list(list);

            // If the current mode is words and images, and the current widget is a Tab, then at
            // this point the user can be navigated directly to the Image tab.
            if matches!(state.base.mode, Mode::WordsAndImages) {
                if let Widget::Tabs(tab) = state.widget.get().unwrap_ji() {
                    tab.set(Tab::new(state.clone(), MenuTabKind::Image));
                }
            }
        }),
        config::get_single_list_init_word,
    );

    let options = SingleListOptions {
        max_rows: shared_config::MAX_LIST_WORDS,
        min_valid: shared_config::MIN_LIST_WORDS,
    };

    SingleListState::new(options, callbacks)
}
fn make_dual_list<RawData: RawDataExt, E: ExtraExt>(
    state: Rc<Step1<RawData, E>>,
) -> DualListState {
    let mode = state.base.mode;

    let callbacks = DualListCallbacks::new(
        |text| super::actions::limit_text(config::DUAL_LIST_CHAR_LIMIT, text),
        clone!(state => move |tooltip| {
            state.base.tooltips.list_error.set(tooltip);
        }),
        clone!(state => move |list| {
            state.base.replace_dual_list(list);
        }),
        config::get_dual_list_init_word,
        clone!(mode => move |side| {
            strings::STR_HEADER(side, mode).to_string()
        }),
    );

    let options = DualListOptions {
        max_rows: shared_config::MAX_LIST_WORDS,
        cell_rows: {
            match state.base.mode {
                Mode::Riddles => 2,
                _ => 1,
            }
        },
        min_valid: shared_config::MIN_LIST_WORDS,
    };

    DualListState::new(options, callbacks)
}
