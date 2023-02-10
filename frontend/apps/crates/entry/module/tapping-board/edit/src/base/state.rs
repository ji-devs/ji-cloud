use components::image::tag::ImageTag;
use components::module::_common::edit::prelude::*;

use components::module::_groups::design::edit::design_ext::DesignExt;
use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{Sticker, Stickers},
    },
    text_editor::{TextEditor, TextEditorCallbacks},
    traces::{
        bubble::TraceBubble,
        edit::{TracesEdit, TracesEditCallbacks},
    },
};
use dominator::clone;
use futures_signals::signal::{always, Signal};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable},
    signal_vec::MutableVec,
};
use shared::domain::module::body::BodyExt;
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            ModuleAssist,
            _groups::design::TraceKind,
            tapping_board::{
                Hint, Mode, ModuleData as RawData, Next, PlaySettings as RawPlaySettings, Step,
            },
        },
        ModuleId,
    },
};
use std::cell::RefCell;
use std::rc::Rc;
use utils::prelude::*;
pub struct Base {
    pub history: Rc<HistoryStateImpl<RawData>>,
    pub step: ReadOnlyMutable<Step>,
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<ModuleAssist>,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    // TappingBoard-specific
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub traces: Rc<TracesEdit>,
    pub traces_meta: MutableVec<TraceMeta>,
    pub text_editor: Rc<TextEditor>,
    pub play_settings: Rc<PlaySettings>,
    pub continue_next_fn: ContinueNextFn,
}

pub struct PlaySettings {
    pub hint: Mutable<Hint>,
    pub next: Mutable<Next>,
}

impl PlaySettings {
    pub fn new(settings: RawPlaySettings) -> Self {
        Self {
            hint: Mutable::new(settings.hint),
            next: Mutable::new(settings.next),
        }
    }
}

#[derive(Clone)]
pub struct TraceMeta {
    pub bubble: Mutable<Option<Rc<TraceBubble>>>,
}

impl TraceMeta {
    pub fn new() -> Self {
        Self {
            bubble: Mutable::new(None),
        }
    }
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            asset_id,
            module_id,
            history,
            step,
            theme_id,
            ..
        } = init_args;

        let content = raw.content.unwrap_ji();

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let instructions = Mutable::new(content.base.instructions);

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
            &content.base.backgrounds,
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
            content
                .base
                .stickers
                .iter()
                .map(|raw_sticker| Sticker::new(stickers.clone(), raw_sticker))
                .collect::<Vec<Sticker>>(),
        );

        *stickers_ref.borrow_mut() = Some(stickers.clone());

        let traces = TracesEdit::from_raw(
            &content.traces,
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

        let traces_meta = MutableVec::new_with_values(
            content
                .traces
                .iter()
                .map(|_trace_meta| TraceMeta::new())
                .collect(),
        );

        let _self = Rc::new(Self {
            asset_id,
            module_id,
            theme_id,
            history,
            step: step.read_only(),
            instructions,
            text_editor,
            backgrounds,
            stickers,
            traces,
            traces_meta,
            play_settings: Rc::new(PlaySettings::new(content.play_settings)),
            continue_next_fn: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }
}

impl BaseExt<Step> for Base {
    type CanContinueSignal = impl Signal<Item = bool>;
    fn allowed_step_change(&self, _from: Step, _to: Step) -> bool {
        true
    }

    fn can_continue_next(&self) -> Self::CanContinueSignal {
        always(true)
    }

    fn continue_next(&self) -> bool {
        match self.step.get() {
            Step::Two | Step::Three | Step::Four => match self.continue_next_fn.get_cloned() {
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
            Mode::Words => vec![ImageTag::Boards],
            Mode::Images => vec![],
            Mode::Talk => vec![],
            Mode::Read => vec![ImageTag::Book],
            Mode::Scene => vec![],
            Mode::PhotoAlbum => vec![ImageTag::PhotoAlbum],
            Mode::Comic => vec![ImageTag::Comix],
            Mode::Timeline => vec![ImageTag::Timeline],
            Mode::FamilyTree => vec![ImageTag::PhotoAlbum],
        })
    }
}
