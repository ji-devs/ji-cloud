use dominator::{html, clone, Dom};
use futures_signals::signal::Mutable;
use shared::domain::jig::ModuleKind;
use super::module::dom::ModuleDom;

static MODULE_KINDS:&[ModuleKind] = &[
    //ModuleKind::Cover,
    ModuleKind::Flashcards,
    ModuleKind::Matching,
    ModuleKind::Memory,
    ModuleKind::Poster,
    ModuleKind::TappingBoard,
    ModuleKind::Tracing,
    ModuleKind::Video,
    ModuleKind::VisualQuiz
];

pub struct SelectionDom {
}

impl SelectionDom {
    pub fn render() -> Dom {
        html!("jig-edit-selection", {
            .property("slot", "selection")
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
