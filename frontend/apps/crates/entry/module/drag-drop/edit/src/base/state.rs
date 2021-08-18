use components::{module::_common::edit::prelude::*, stickers::state::Sticker};
use components::audio::mixer::AudioMixer;
use dominator_helpers::signals::OptionSignal;
use utils::drag::Drag;
use uuid::Uuid;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            StepExt,
            ThemeChoice,
            Audio,
            Instructions,
            Transform,
            drag_drop::{
                Step,
                PlaySettings as RawPlaySettings, 
                Hint, Next,
                Mode, 
                Item as RawItem,
                ItemKind as RawItemKind,
                Interactive as RawInteractive,
                TargetArea as RawTargetArea,
                Content as RawContent, 
                ModuleData as RawData
            },
            _groups::design::{
                Trace as RawTrace,
                Sticker as RawSticker,
                Backgrounds as RawBackgrounds, 
            }
        }
    }
};
use futures_signals::{
    map_ref,
    signal::{self, Signal, SignalExt, ReadOnlyMutable, Mutable},
    signal_vec::{MutableVec, SignalVecExt},
};
use utils::prelude::*;
use components::{
    text_editor::{
        state::State as TextEditorState,
        callbacks::Callbacks as TextEditorCallbacks
    },
    stickers::{
        state::{Stickers, AsSticker},
        callbacks::Callbacks as StickersCallbacks
    },
    backgrounds::{
        state::Backgrounds,
        callbacks::Callbacks as BackgroundsCallbacks,
    },
    traces::{
        bubble::state::TraceBubble,
        edit::{
            state::TracesEdit, 
            callbacks::Callbacks as TracesCallbacks
        }
    },
    tooltip::state::State as TooltipState
};
use dominator::clone;
use std::cell::RefCell;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    // DragDrop-specific
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers<Item>>, 
    pub traces: Rc<TracesEdit>,
    pub targets_meta: Mutable<Vec<TargetMeta>>,
    pub text_editor: Rc<TextEditorState>,
    pub audio_mixer: AudioMixer,
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
    pub fn new(settings:RawPlaySettings) -> Self {
        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
            time_limit: Mutable::new(settings.time_limit.unwrap_or(crate::config::DEFAULT_TIME_LIMIT)),
            has_time_limit: Mutable::new(settings.time_limit.is_some()),
        }
    }

}

#[derive(Clone)]
pub struct Item {
    pub sticker: Sticker,
    pub kind: Mutable<ItemKind>
}

impl Item {
    pub fn new(stickers: Rc<Stickers<Item>>, raw:&RawItem) -> Self {
        Self {
            sticker: Sticker::new(stickers, &raw.sticker),
            kind: Mutable::new(
                      match raw.kind.clone() {
                        RawItemKind::Static => ItemKind::Static,
                        RawItemKind::Interactive(data) => {
                            ItemKind::Interactive(Interactive::new(data))
                        }
                      }
            )
        }
    }

    pub fn to_raw(&self) -> RawItem {
        RawItem {
            sticker: self.sticker.to_raw(),
            kind: match self.kind.get_cloned() {
                    ItemKind::Static => RawItemKind::Static,
                    ItemKind::Interactive(data) => {
                        RawItemKind::Interactive(RawInteractive{
                            audio: data.audio.get_cloned(),
                            target_transform: data.target_transform.get_cloned()
                        })
                    }
            }
        }
    }


    pub fn get_interactive_unchecked(&self) -> Interactive {
        match self.kind.get_cloned() {
            ItemKind::Interactive(data) => {
                data
            }
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
            kind: Mutable::new(ItemKind::Static)
        }
    }
    fn duplicate_with_sticker(&self, sticker: Sticker) -> Self {
        Self {
            sticker,
            kind: Mutable::new(self.kind.get_cloned())
        }
    }
}

impl AsRef<Sticker> for Item {
    fn as_ref(&self) -> &Sticker {
        &self.sticker
    }
}

#[derive(Clone)]
pub struct TargetMeta {
    pub id: Uuid,
}

impl TargetMeta {
    pub fn new(raw: Option<&RawTargetArea>) -> Self {
        Self {
            id: match raw {
                Some(raw) => raw.id,
                None => Uuid::new_v4() 
            }
        }
    }
}


#[derive(Clone)]
pub enum ItemKind {
    Static,
    Interactive(Interactive)
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
            theme_choice,
            theme_id,
            audio_mixer,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.instructions);
        let feedback = Mutable::new(content.feedback);
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers<Item>>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(
            theme_id.clone(),
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
                }))
        ));


        let backgrounds = Rc::new(Backgrounds::from_raw(
                &content.backgrounds,
                theme_id.clone(),
                BackgroundsCallbacks::new(
                    Some(clone!(history => move |raw_bgs| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.backgrounds = raw_bgs;
                            }
                        });
                    }))
                )
        ));

        let stickers = Stickers::new(
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |items:&[Item]| {
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
                    }))
                )
        );
       
        stickers.replace_all(
            content.items
                .iter()
                .map(|item| {
                    Item::new(stickers.clone(), item)
                })
                .collect::<Vec<Item>>()
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let traces = TracesEdit::from_raw(

            &content.target_areas
                .iter()
                .map(|target_area| {
                    target_area.trace.clone()
                })
                .collect::<Vec<RawTrace>>(),
            crate::debug::settings().trace_opts.clone(),
            TracesCallbacks::new(
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
            )
        );

        let targets_meta = Mutable::new(
            content.target_areas
                .iter()
                .map(|target_area| {
                    TargetMeta::new(Some(target_area))
                })
                .collect()
        );

        let _self = Rc::new(Self {
            jig_id,
            module_id,
            theme_id,
            history,
            step: step.read_only(),
            theme_choice,
            instructions,
            feedback,
            text_editor,
            backgrounds,
            stickers,
            traces,
            targets_meta,
            audio_mixer,
            play_settings: Rc::new(PlaySettings::new(content.play_settings.clone())),
            drag_item_selected_index: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> { 
        self.theme_id.signal().map(|id| id.as_str_id())
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
                OptionSignal::new(
                    index_item.map(|(index, item)| {
                        item.kind.signal_cloned()
                            .map(clone!(index => move |kind| (index, kind)))
                    })
                )
            })
            .flatten()
    }

}


impl BaseExt<Step> for Base {
    type NextStepAllowedSignal = impl Signal<Item = bool>;

    fn allowed_step_change(&self, from:Step, to:Step) -> bool {
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
