use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{Sticker, Stickers},
    },
    text_editor::{callbacks::Callbacks as TextEditorCallbacks, state::State as TextEditorState},
};
use components::{module::_common::edit::prelude::*, stickers::video::state::Video};
use dominator::clone;
use futures_signals::signal::{self, Mutable, ReadOnlyMutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVecExt, VecDiff};
use shared::domain::jig::{
    module::{
        body::{
            video::{
                DoneAction, Mode, ModuleData as RawData, PlaySettings as RawPlaySettings, Step,
            },
            Instructions, ThemeChoice,
        },
        ModuleId,
    },
    JigId,
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_choice: Mutable<ThemeChoice>,
    pub theme_id: ReadOnlyMutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub jig_theme_id: Mutable<ThemeId>,
    // Video-specific
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub text_editor: Rc<TextEditorState>,
    pub play_settings: PlaySettings,

    // reference to the video in the stickers list
    pub video: Mutable<Option<Rc<Video>>>,
}

pub struct PlaySettings {
    pub captions: Mutable<bool>,
    pub muted: Mutable<bool>,
    pub autoplay: Mutable<bool>,
    pub done_action: Mutable<Option<DoneAction>>,
}

impl PlaySettings {
    pub fn new(settings: RawPlaySettings) -> Self {
        Self {
            captions: Mutable::new(settings.captions),
            muted: Mutable::new(settings.muted),
            autoplay: Mutable::new(settings.autoplay),
            done_action: Mutable::new(settings.done_action),
        }
    }

    pub fn to_raw(&self) -> RawPlaySettings {
        RawPlaySettings {
            captions: self.captions.get_cloned(),
            muted: self.muted.get_cloned(),
            autoplay: self.autoplay.get_cloned(),
            done_action: self.done_action.get_cloned(),
        }
    }
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            jig_id,
            jig_theme_id,
            theme_id,
            module_id,
            history,
            step,
            theme_choice,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(base_content.instructions);

        let stickers_ref: Rc<RefCell<Option<Rc<Stickers<Sticker>>>>> = Rc::new(RefCell::new(None));

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
                })),
            ),
        );

        let backgrounds = Rc::new(Backgrounds::from_raw(
            &base_content.backgrounds,
            theme_id.clone(),
            BackgroundsCallbacks::new(Some(clone!(history => move |raw_bgs| {
                history.push_modify(|raw| {
                    if let Some(content) = &mut raw.content {
                        content.base.backgrounds = raw_bgs;
                    }
                });
            }))),
        ));

        let stickers = Stickers::new(
            text_editor.clone(),
            StickersCallbacks::new(Some(clone!(history => move |stickers:&[Sticker]| {
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
            }))),
        );

        stickers.replace_all(
            base_content
                .stickers
                .clone()
                .iter()
                .map(|raw_sticker| Sticker::new(stickers.clone(), raw_sticker))
                .collect::<Vec<Sticker>>(),
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
            play_settings: PlaySettings::new(content.play_settings.clone()),
            video: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        // this listens for changes in the stickers list and updates _self.video whenever there is a video
        spawn_local(_self.stickers.list.signal_vec_cloned().for_each(
            clone!(_self => move |diff| {
                match diff {
                    VecDiff::Replace {..} |
                    VecDiff::RemoveAt {..} |
                    VecDiff::Pop {..} => {
                        // check if video in vec
                        let stickers = _self.stickers.list.lock_ref();
                        let video_sticker = stickers.iter().find(|sticker| {
                            matches!(sticker, Sticker::Video(_))
                        });

                        match video_sticker {
                            None => {
                                _self.video.set(None);
                            },
                            Some(Sticker::Video(video)) => {
                                _self.video.set(Some(Rc::clone(video)));
                            },
                            _ => unreachable!(),
                        }
                    },

                    VecDiff::InsertAt { value, .. } |
                    VecDiff::UpdateAt { value, .. } |
                    VecDiff::Push { value } => {
                        // if value is a video, set
                        if let Sticker::Video(video) = value {
                            _self.video.set(Some(video));
                        };
                    },
                    VecDiff::Clear { .. } => {
                        // remove video
                        _self.video.set(None);
                    },

                    VecDiff::Move { .. } => {
                        // do nothing
                    },

                };
                async {}
            }),
        ));

        _self
    }

    pub fn theme_id_str_signal(&self) -> impl Signal<Item = &'static str> {
        self.theme_id.signal().map(|id| id.as_str_id())
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
