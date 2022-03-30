use components::{
    module::{_common::edit::prelude::*, _groups::design::edit::design_ext::DesignExt},
    stickers::state::Sticker,
};

use dominator_helpers::signals::OptionSignal;

use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{AsSticker, Stickers},
    },
    text_editor::{callbacks::Callbacks as TextEditorCallbacks, state::State as TextEditorState},
    traces::edit::{TracesEdit, TracesEditCallbacks},
};
use dominator::clone;
use futures_signals::{
    map_ref,
    signal::{self, Mutable, ReadOnlyMutable, Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::jig::{
    module::{
        body::{
            Audio, Instructions, Transform,
            _groups::design::{Trace as RawTrace, TraceKind},
            drag_drop::{
                Hint, Interactive as RawInteractive, Item as RawItem, ItemKind as RawItemKind,
                Mode, ModuleData as RawData, Next, PlaySettings as RawPlaySettings, Step,
            },
            BodyExt,
        },
        ModuleId,
    },
    JigId,
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    // DragDrop-specific
    pub theme_id: Mutable<ThemeId>,
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Item>>,
    pub traces: Rc<TracesEdit>,
    pub text_editor: Rc<TextEditorState>,
    pub play_settings: Rc<PlaySettings>,

    pub drag_item_selected_index: Mutable<Option<usize>>,
    pub feedback: Mutable<Instructions>,
}

pub struct PlaySettings {
    pub hint: Mutable<Hint>,
    pub next: Mutable<Next>,
    pub time_limit: Mutable<u32>,
    pub has_time_limit: Mutable<bool>,
}

impl PlaySettings {
    pub fn new(settings: RawPlaySettings) -> Self {
        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
            time_limit: Mutable::new(
                settings
                    .time_limit
                    .unwrap_or(crate::config::DEFAULT_TIME_LIMIT),
            ),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
        }
    }
}

#[derive(Clone)]
pub struct Item {
    pub sticker: Sticker,
    pub kind: Mutable<ItemKind>,
}

impl Item {
    pub fn new(stickers: Rc<Stickers<Item>>, raw: &RawItem) -> Self {
        Self {
            sticker: Sticker::new(stickers, &raw.sticker),
            kind: Mutable::new(match raw.kind.clone() {
                RawItemKind::Static => ItemKind::Static,
                RawItemKind::Interactive(data) => ItemKind::Interactive(Interactive::new(data)),
            }),
        }
    }

    pub fn to_raw(&self) -> RawItem {
        RawItem {
            sticker: self.sticker.to_raw(),
            kind: match self.kind.get_cloned() {
                ItemKind::Static => RawItemKind::Static,
                ItemKind::Interactive(data) => RawItemKind::Interactive(RawInteractive {
                    audio: data.audio.get_cloned(),
                    target_transform: data.target_transform.get_cloned(),
                }),
            },
        }
    }

    pub fn get_interactive_unchecked(&self) -> Interactive {
        match self.kind.get_cloned() {
            ItemKind::Interactive(data) => data,
            _ => {
                panic!("failed to get interactive data!");
            }
        }
    }
}

impl AsSticker for Item {
    fn new_from_sticker(sticker: Sticker) -> Self {
        Self {
            sticker,
            kind: Mutable::new(ItemKind::Static),
        }
    }
    fn duplicate_with_sticker(&self, sticker: Sticker) -> Self {
        Self {
            sticker,
            kind: Mutable::new(self.kind.get_cloned()),
        }
    }
}

impl AsRef<Sticker> for Item {
    fn as_ref(&self) -> &Sticker {
        &self.sticker
    }
}

#[derive(Clone)]
pub enum ItemKind {
    Static,
    Interactive(Interactive),
}

#[derive(Clone)]
pub struct Interactive {
    pub audio: Mutable<Option<Audio>>,
    pub target_transform: Mutable<Option<Transform>>,
}

impl Interactive {
    pub fn new(raw: RawInteractive) -> Self {
        Self {
            audio: Mutable::new(raw.audio),
            target_transform: Mutable::new(raw.target_transform),
        }
    }
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            jig_id,
            module_id,
            history,
            step,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.instructions);
        let feedback = Mutable::new(content.feedback);

        let stickers_ref: Rc<RefCell<Option<Rc<Stickers<Item>>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(
            theme_id.read_only(),
            None,
            TextEditorCallbacks::new(
                //New text
                Some(clone!(stickers_ref => move |value:&str| {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        Stickers::add_text(stickers.clone(), value.to_string());
                    }
                })),
                //Text change
                Some(clone!(stickers_ref => move |value:&str| {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        stickers.set_current_text_value(value.to_string());
                    }
                })),
                //Blur
                Some(clone!(stickers_ref => move || {
                    if let Some(stickers) = stickers_ref.borrow().as_ref() {
                        stickers.stop_current_text_editing();
                    }
                })),
            ),
        );

        let backgrounds = Rc::new(Backgrounds::from_raw(
            &content.backgrounds,
            theme_id.read_only(),
            BackgroundsCallbacks::new(Some(clone!(history => move |raw_bgs| {
                history.push_modify(|raw| {
                    if let Some(content) = &mut raw.content {
                        content.backgrounds = raw_bgs;
                    }
                });
            }))),
        ));

        let stickers = Stickers::new(
            text_editor.clone(),
            StickersCallbacks::new(Some(clone!(history => move |items:&[Item]| {
                history.push_modify(|raw| {
                    if let Some(content) = &mut raw.content {
                        content.items = items
                            .iter()
                            .map(|item| {
                                item.to_raw()
                            })
                            .collect();
                    }
                });
            }))),
        );

        stickers.replace_all(
            content
                .items
                .iter()
                .map(|item| Item::new(stickers.clone(), item))
                .collect::<Vec<Item>>(),
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let traces = TracesEdit::from_raw(
            &content
                .target_areas
                .iter()
                .map(|target_area| target_area.trace.clone())
                .collect::<Vec<RawTrace>>(),
            crate::debug::settings()
                .draw_kind
                .unwrap_or(TraceKind::Regular),
            TracesEditCallbacks::new(
                Some(clone!(_self_ref => move |raw_trace| {
                    if let Some(_self) = _self_ref.borrow().as_ref() {
                        _self.on_trace_added(raw_trace);
                    }
                })),
                Some(clone!(_self_ref => move |index| {
                    if let Some(_self) = _self_ref.borrow().as_ref() {
                        _self.on_trace_deleted(index);
                    }
                })),
                Some(clone!(_self_ref => move |index, raw_trace| {
                    if let Some(_self) = _self_ref.borrow().as_ref() {
                        _self.on_trace_changed(index, raw_trace);
                    }
                })),
            ),
        );

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            theme_id,
            history,
            step: step.read_only(),
            instructions,
            feedback,
            text_editor,
            backgrounds,
            stickers,
            traces,
            play_settings: Rc::new(PlaySettings::new(content.play_settings)),
            drag_item_selected_index: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn selected_item_signal(&self) -> impl Signal<Item = Option<(usize, Item)>> {
        map_ref! {
            let index = self.drag_item_selected_index.signal(),
            let list = self.stickers.list.signal_vec_cloned().to_signal_cloned()
                => {
                    index.and_then(|index| {
                        list
                            .get(index)
                            .map(|item| (index, item.clone()))
                    })
                }
        }
    }

    pub fn selected_item_kind_signal(&self) -> impl Signal<Item = Option<(usize, ItemKind)>> {
        self.selected_item_signal()
            .map(|index_item| {
                OptionSignal::new(index_item.map(|(index, item)| {
                    item.kind
                        .signal_cloned()
                        .map(clone!(index => move |kind| (index, kind)))
                }))
            })
            .flatten()
    }
}

impl BaseExt<Step> for Base {
    type NextStepAllowedSignal = impl Signal<Item = bool>;

    fn allowed_step_change(&self, _from: Step, _to: Step) -> bool {
        true
    }

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal {
        signal::always(true)
    }

    fn get_jig_id(&self) -> JigId {
        self.jig_id
    }
    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }
}

impl DesignExt for Base {
    fn get_backgrounds(&self) -> Rc<Backgrounds> {
        Rc::clone(&self.backgrounds)
    }

    fn get_theme(&self) -> Mutable<ThemeId> {
        self.theme_id.clone()
    }

    fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set(theme.clone());

        self.history.push_modify(|raw| {
            raw.set_theme(theme);
        });
    }
}
