use components::module::_common::edit::prelude::*;
use components::audio_mixer::AudioMixer;
use dominator_helpers::signals::OptionSignal;
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
    signal_vec::MutableVec
};
use utils::prelude::*;
use components::{
    text_editor::{
        state::State as TextEditorState,
        callbacks::Callbacks as TextEditorCallbacks
    },
    stickers::{
        state::Stickers,
        callbacks::Callbacks as StickersCallbacks
    },
    backgrounds::{
        state::Backgrounds,
        callbacks::Callbacks as BackgroundsCallbacks,
    },
    traces::{
        bubble::state::TraceBubble,
        edit::{
            state::Edit as TracesEdit, 
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
    pub stickers: Rc<Stickers>, 
    pub items_meta: Mutable<Vec<ItemMeta>>,
    pub traces: Rc<TracesEdit>,
    pub targets_meta: Mutable<Vec<TargetMeta>>,
    pub text_editor: Rc<TextEditorState>,
    pub audio_mixer: AudioMixer,
    pub play_settings: Rc<PlaySettings>,
}

pub struct PlaySettings {
    pub hint: Mutable<Hint>,
    pub next: Mutable<Next>,
    pub next_value: Mutable<usize>
}

impl PlaySettings {
    pub fn new(settings:RawPlaySettings) -> Self {

        let next_value = Mutable::new(
                match &settings.next {
                    Next::SelectSome(value) => *value,
                    _ => {
                        crate::config::DEFAULT_SELECT_AMOUNT
                    }
                },
            );
        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
            next_value
        }
    }

    pub fn to_raw(&self) -> RawPlaySettings {
        RawPlaySettings {
            hint: self.hint.get_cloned(),
            next: self.next.get_cloned(),
        }
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
pub struct ItemMeta {
    pub kind: Mutable<ItemKind>
}

impl ItemMeta {
    pub fn new(raw: Option<&RawItem>) -> Self {
        Self {
            kind: Mutable::new(
              match raw {
                  None => ItemKind::Static,
                  Some(raw) => {
                      match raw.kind.clone() {
                        RawItemKind::Static => ItemKind::Static,
                        RawItemKind::Interactive(data) => {
                            ItemKind::Interactive(Interactive {
                                audio: Mutable::new(data.audio),
                                target_id: Mutable::new(data.target_id),
                            })
                        }
                      }
                  }
              }
            )
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
    pub target_id: Mutable<Option<Uuid>>,
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
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers>>>> = Rc::new(RefCell::new(None));

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

        let stickers = Stickers::from_raw(
                &content.items
                    .iter()
                    .map(|item| {
                        item.sticker.clone()
                    })
                    .collect::<Vec<RawSticker>>(),
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |raw_stickers| {
                        //TODO - need to split into add/delete/change, like trace...
                    }))
                )
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let items_meta = Mutable::new(
            content.items
                .iter()
                .map(|item| {
                    ItemMeta::new(Some(&item))
                })
                .collect()
        );


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
            text_editor,
            backgrounds,
            stickers,
            items_meta,
            traces,
            targets_meta,
            audio_mixer,
            play_settings: Rc::new(PlaySettings::new(content.play_settings.clone())),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> { 
        self.theme_id.signal().map(|id| id.as_str_id())
    }

    //TODO - these can/should be made simpler by maintaining a "selected for drag" index
    //independent of stickers.selected_index
    pub fn selected_item_meta_signal(&self) -> impl Signal<Item = Option<(usize, ItemMeta)>> {
        map_ref! {
            let index = self.stickers.selected_index.signal(),
            let items_meta = self.items_meta.signal_cloned()
                => {
                    index.and_then(|index| {
                        items_meta
                            .get(index)
                            .map(|meta| (index, meta.clone()))
                    })
                }
        }
    }

    pub fn selected_item_kind_signal(&self) -> impl Signal<Item = Option<(usize, ItemKind)>> {
        self.selected_item_meta_signal()
            .map(|index_meta| {
                OptionSignal::new(
                    index_meta.map(|(index, meta)| {
                        meta.kind.signal_cloned()
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


}
