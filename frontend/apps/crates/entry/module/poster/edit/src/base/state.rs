use components::module::_common::edit::prelude::*;

use components::module::_groups::design::edit::design_ext::DesignExt;
use components::{
    backgrounds::{callbacks::Callbacks as BackgroundsCallbacks, state::Backgrounds},
    stickers::{
        callbacks::Callbacks as StickersCallbacks,
        state::{Sticker, Stickers},
    },
    text_editor::{callbacks::Callbacks as TextEditorCallbacks, state::State as TextEditorState},
};
use dominator::clone;
use futures_signals::signal::{Mutable, ReadOnlyMutable};
use shared::domain::jig::module::body::BodyExt;
use shared::domain::jig::{
    module::{
        body::{
            poster::{Mode, ModuleData as RawData, Step},
            Instructions,
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
    pub theme_id: Mutable<ThemeId>,
    pub instructions: Mutable<Instructions>,
    pub jig_id: JigId,
    pub module_id: ModuleId,
    pub continue_next_fn: ContinueNextFn,
    // Poster-specific
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub text_editor: Rc<TextEditorState>,
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, Mode, Step>) -> Rc<Self> {
        let BaseInitFromRawArgs {
            raw,
            jig_id,
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
            jig_id,
            module_id,
            history,
            step: step.read_only(),
            theme_id,
            instructions,
            text_editor,
            backgrounds,
            stickers,
            continue_next_fn: Mutable::new(None),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }
}

impl BaseExt<Step> for Base {
    fn allowed_step_change(&self, _from: Step, _to: Step) -> bool {
        true
    }

    fn can_continue_next(&self) -> ReadOnlyMutable<bool> {
        Mutable::new(true).read_only()
    }

    fn continue_next(&self) -> bool {
        match self.step.get() {
            Step::Two => match self.continue_next_fn.get_cloned() {
                Some(continue_next_fn) => continue_next_fn(),
                None => false,
            },
            _ => false,
        }
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
        self.theme_id.set(theme);

        self.history.push_modify(|raw| {
            raw.set_theme(theme);
        });
    }
}
