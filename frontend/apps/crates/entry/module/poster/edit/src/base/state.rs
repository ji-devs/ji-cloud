use components::module::edit::prelude::*;
use components::audio_mixer::AudioMixer;
use std::rc::Rc;
use shared::domain::jig::{
    JigId, 
    Jig,
    module::{
        ModuleId, 
        body::{
            ThemeChoice,
            Backgrounds as RawBackgrounds, 
            Audio,
            Instructions,
            poster::{
                Step,
                Mode,
                Content as RawContent, 
                ModuleData as RawData
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
    tooltip::state::State as TooltipState
};
use dominator::clone;
use std::cell::RefCell;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme: Mutable<ThemeChoice>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig_theme_id: Mutable<ThemeId>,
    // Poster-specific
    pub backgrounds: Rc<Backgrounds>, 
    pub stickers: Rc<Stickers>, 
    pub text_editor: Rc<TextEditorState>,
    pub audio_mixer: AudioMixer,
}


impl Base {

    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {

        let BaseInitFromRawArgs { 
            raw,
            jig_id,
            jig_theme_id,
            module_id,
            history,
            step,
            theme,
            audio_mixer,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref:Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.instructions);
      
        let stickers_ref:Rc<RefCell<Option<Rc<Stickers>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditorState::new(

            match content.theme {
                ThemeChoice::Jig => {
                    // self.jig.as_ref().unwrap_ji().theme_id.clone()
                    log::warn!("waiting on jig settings");
                    ThemeId::Chalkboard
                },
                ThemeChoice::Override(theme_id) => theme_id
            },
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
                &content.stickers,
                text_editor.clone(),
                StickersCallbacks::new(
                    Some(clone!(history => move |raw_stickers| {
                        history.push_modify(|raw| {
                            if let Some(content) = &mut raw.content {
                                content.stickers = raw_stickers;
                            }
                        });
                    }))
                )
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());



        let _self = Rc::new(Self {
            jig_id,
            module_id,
            jig_theme_id,
            history,
            step: step.read_only(),
            theme,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            audio_mixer,
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

}

impl BaseExt<Step> for Base {
    type NextStepAllowedSignal = impl Signal<Item = bool>;
    type ThemeIdSignal = impl Signal<Item = ThemeId>;
    type ThemeIdStrSignal = impl Signal<Item = &'static str>;

    fn allowed_step_change(&self, from:Step, to:Step) -> bool {
        true
    }

    fn next_step_allowed_signal(&self) -> Self::NextStepAllowedSignal {
        signal::always(true)
    }

    fn get_theme_id(&self) -> ThemeId {
        match self.theme.get_cloned() {
            ThemeChoice::Jig => self.jig_theme_id.get(),
            ThemeChoice::Override(theme_id) => theme_id
        }
    }
    fn theme_id_signal(&self) -> Self::ThemeIdSignal { 
        map_ref! {
            let jig_theme_id = self.jig_theme_id.signal(),
            let theme = self.theme.signal()
                => {
                match *theme { 
                    ThemeChoice::Jig => *jig_theme_id,
                    ThemeChoice::Override(theme_id) => theme_id
                }
            }
        }
    }

    fn theme_id_str_signal(&self) -> Self::ThemeIdStrSignal { 
        self.theme_id_signal().map(|id| id.as_str_id())
    }
}
