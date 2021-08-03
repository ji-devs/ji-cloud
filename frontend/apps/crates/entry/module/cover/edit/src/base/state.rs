use components::module::_common::edit::prelude::*;
use components::audio_mixer::AudioMixer;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Audio,
            Instructions,
            _groups::design::Backgrounds as RawBackgrounds, 
            cover::{
                Step,
                Content as RawContent, 
                ModuleData as RawData
            },

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
        state::{Stickers, Sticker},
        callbacks::Callbacks as StickersCallbacks
    },
    backgrounds::{
        state::Backgrounds,
        callbacks::Callbacks as BackgroundsCallbacks,
    },
    tooltip::state::State as TooltipState
};
use dominator::clone;
use std::cell::RefCell;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig_theme_id: Mutable<ThemeId>,
    // Cover-specific
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers<Sticker>>, 
    pub text_editor: Rc<TextEditorState>,
    pub audio_mixer: AudioMixer,
}


impl Base {

    pub async fn new(init_args: BaseInitFromRawArgs<RawData, (), Step>) -> Rc<Self> {

        let BaseInitFromRawArgs { 
            raw,
            jig_id,
            jig_theme_id,
            theme_id,
            module_id,
            history,
            step,
            theme_choice,
            audio_mixer,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base; 

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(base_content.instructions);
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers<Sticker>>>>> = Rc::new(RefCell::new(None));

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
                &base_content.backgrounds,
                theme_id.clone(),
                BackgroundsCallbacks::new(
                    Some(clone!(history => move |raw_bgs| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.base.backgrounds = raw_bgs;
                            }
                        });
                    }))
                )
        ));

        let stickers = Stickers::new(
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |stickers:&[Sticker]| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.base.stickers = stickers 
                                    .iter()
                                    .map(|sticker| {
                                        sticker.to_raw()
                                    })
                                    .collect();
                            }
                        });
                    }))
                )
        );
       
        stickers.replace_all(
            base_content.stickers.clone()
                .iter()
                .map(|raw_sticker| {
                    Sticker::new(stickers.clone(), raw_sticker)
                })
                .collect::<Vec<Sticker>>()
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());



        let _self = Rc::new(Self {
            jig_id,
            module_id,
            jig_theme_id,
            history,
            step: step.read_only(),
            theme_choice,
            theme_id,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            audio_mixer,
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> { 
        self.theme_id.signal().map(|id| id.as_str_id())
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
