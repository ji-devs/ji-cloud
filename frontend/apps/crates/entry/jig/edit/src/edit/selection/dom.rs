use components::jigzi_help::JigziHelp;
use dominator::{html, Dom};

use super::module::dom::ModuleDom;
use shared::domain::jig::ModuleKind;

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
];

const STR_TOOLTIP_TITLE: &str = "Let's build your JIG!";
const STR_TOOLTIP_BODY: &str =
    "Select an activity and drag it to the body of your JIG. You can change the order at any time.";

pub struct SelectionDom {}

impl SelectionDom {
    pub fn render() -> Dom {
        html!("jig-edit-selection", {
            .property("slot", "main")
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
                .render(Some("help"))
            )
        })
    }
}
