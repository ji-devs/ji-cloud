use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    image::tag::ImageTag,
    module::_groups::design::edit::design_ext::DesignExt,
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{Sticker, Stickers},
    },
    text_editor::{TextEditor, TextEditorCallbacks},
};
use components::{module::_common::edit::prelude::*, stickers::video::state::Video};
use dominator::clone;
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use futures_signals::signal_vec::{SignalVecExt, VecDiff};
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            video::{
                DoneAction, Mode, ModuleData as RawData, PlaySettings as RawPlaySettings, Step,
            },
            BodyExt, ModuleAssist,
        },
        ModuleId,
    },
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
use wasm_bindgen_futures::spawn_local;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<ModuleAssist>,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub can_continue_next: Mutable<bool>,
    pub continue_next_fn: ContinueNextFn,
    // Video-specific
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub text_editor: Rc<TextEditor>,
    pub play_settings: PlaySettings,

    // reference to the video in the stickers list
    pub video: Mutable<Option<Rc<Video>>>,
    pub clip: Mutable<bool>,
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
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            asset_id,
            theme_id,
            module_id,
            history,
            step,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();
        let base_content = content.base;

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(base_content.instructions);

        let stickers_ref: Rc<RefCell<Option<Rc<Stickers<Sticker>>>>> = Rc::new(RefCell::new(None));

        let text_editor = TextEditor::new(
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
            &base_content.backgrounds,
            theme_id.read_only(),
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
                .iter()
                .map(|raw_sticker| Sticker::new(stickers.clone(), raw_sticker))
                .collect::<Vec<Sticker>>(),
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let _self = Rc::new(Self {
            asset_id,
            module_id,
            history,
            step: step.read_only(),
            can_continue_next: Mutable::new(false),
            continue_next_fn: Mutable::new(None),
            theme_id,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            play_settings: PlaySettings::new(content.play_settings),
            video: Mutable::new(None),
            clip: Mutable::new(false),
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
                                if video.start_at.get().is_some() || video.end_at.get().is_some() {
                                    _self.clip.set(true);
                                }
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
}

impl BaseExt<Step> for Base {
    fn allowed_step_change(&self, _from: Step, to: Step) -> bool {
        match to {
            // Only allow changing to steps 3 and 4 if the video URL has actually been set.
            Step::Three | Step::Four => self.video.get_cloned().is_some(),
            _ => true,
        }
    }

    fn can_continue_next(&self) -> ReadOnlyMutable<bool> {
        self.can_continue_next.read_only()
    }

    fn continue_next(&self) -> bool {
        match self.step.get() {
            Step::Two | Step::Three => match self.continue_next_fn.get_cloned() {
                Some(continue_next_fn) => continue_next_fn(),
                None => false,
            },
            _ => false,
        }
    }

    fn get_asset_id(&self) -> AssetId {
        self.asset_id
    }
    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }
}

impl DesignExt<Mode> for Base {
    fn get_backgrounds(&self) -> Rc<Backgrounds> {
        Rc::clone(&self.backgrounds)
    }

    fn get_theme(&self) -> Mutable<ThemeId> {
        self.theme_id.clone()
    }

    fn set_theme(&self, theme: ThemeId) {
        self.theme_id.set(theme);

        self.history.push_modify(|raw| {
            raw.set_theme(theme);
        });
    }

    fn get_image_tag_priorities(&self) -> Option<Vec<ImageTag>> {
        let mode = self.history.get_current().mode();
        mode.map(|mode| match mode {
            Mode::Introduction => vec![ImageTag::Video],
            Mode::Story => vec![ImageTag::Book],
            Mode::Song => vec![ImageTag::Music],
            Mode::Howto => vec![ImageTag::Boards],
        })
    }
}
