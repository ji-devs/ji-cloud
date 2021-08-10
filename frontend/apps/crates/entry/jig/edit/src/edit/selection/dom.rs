use dominator::{html, Dom};

use super::module::dom::ModuleDom;
use shared::domain::jig::ModuleKind;

static MODULE_KINDS: &[ModuleKind] = &[
    //ModuleKind::Cover,
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
        })
    }
}
