use components::module::_common::edit::prelude::*;

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
use shared::domain::{
    asset::AssetId,
    module::{
        body::{
            resource_cover::{ModuleData as RawData, Step},
            Instructions,
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
    pub instructions: Mutable<Instructions>,
    pub asset_id: AssetId,
    pub module_id: ModuleId,
    pub continue_next_fn: ContinueNextFn,
    // ResourceCover-specific
    pub backgrounds: Rc<Backgrounds>,
    pub stickers: Rc<Stickers<Sticker>>,
    pub text_editor: Rc<TextEditorState>,
}

impl Base {
    pub async fn new(init_args: BaseInitFromRawArgs<RawData, (), Step>) -> Rc<Self> {
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
            asset_id,
            module_id,
            history,
            step: step.read_only(),
            continue_next_fn: Mutable::new(None),
            theme_id,
            instructions,
            text_editor,
            backgrounds,
            stickers,
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
        // TODO Resource cover doesn't seem to be used(?), always returning false here means that
        // when clicking Continue in a Resource Cover module, the sidebar will navigate to the next
        // step.
        false
    }

    fn get_asset_id(&self) -> AssetId {
        self.asset_id
    }

    fn get_module_id(&self) -> ModuleId {
        self.module_id
    }
}
