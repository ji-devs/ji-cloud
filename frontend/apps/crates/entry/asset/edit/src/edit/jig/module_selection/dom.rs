use std::rc::Rc;

use components::jigzi_help::JigziHelp;
use dominator::{clone, html, Dom};

use super::super::super::state::AssetEditState;
use super::module::dom::ModuleDom;
use shared::domain::module::ModuleKind;
use utils::events;

static MODULE_KINDS: &[ModuleKind] = &[
    ModuleKind::Cover,
    ModuleKind::Flashcards,
    ModuleKind::Matching,
    ModuleKind::Memory,
    ModuleKind::CardQuiz,
    ModuleKind::Poster,
    ModuleKind::TappingBoard,
    ModuleKind::DragDrop,
    //ModuleKind::Tracing,
    ModuleKind::Video,
    //ModuleKind::VisualQuiz,
    ModuleKind::FindAnswer,
];

const STR_TOOLTIP_TITLE: &str = "Let's build your JIG!";
const STR_TOOLTIP_BODY: &str =
    "Select an activity and drag it to the body of your JIG. You can change the order at any time.";
const STR_SHOW_ONBOARDING: &str = "Take the tour";

pub struct SelectionDom {}

impl SelectionDom {
    pub fn render(state: Rc<AssetEditState>) -> Dom {
        html!("jig-edit-selection", {
            .prop("slot", "main")
            .children(
                MODULE_KINDS
                    .iter()
                    .map(|module_kind| {
                        ModuleDom::render(*module_kind)
                    })
                    .collect::<Vec<Dom>>()
            )
            .child(
                JigziHelp::new(
                    STR_TOOLTIP_TITLE.to_string(),
                    STR_TOOLTIP_BODY.to_string(),
                    "module-select"
                )
                .render(
                    Some("help"),
                    Rc::new(Some(move || {
                        html!("button-rect", {
                            .prop("kind", "text")
                            .prop("color", "lightBlue")
                            .text(STR_SHOW_ONBOARDING)
                            .event(clone!(state => move |_evt: events::Click| {
                                state.show_onboarding.set_neq(true);
                            }))
                        })
                    }))
                )
            )
        })
    }
}
