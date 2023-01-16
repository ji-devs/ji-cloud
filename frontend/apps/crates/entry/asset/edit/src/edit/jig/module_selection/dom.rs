use std::rc::Rc;

use super::item::ModuleSelectionItem;
use super::ModuleSelection;
use components::jigzi_help::JigziHelp;
use dominator::{clone, html, Dom};
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

impl ModuleSelection {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("asset-edit-jig-selection", {
            .prop("slot", "main")
            .children(
                MODULE_KINDS
                    .iter()
                    .map(|module_kind| {
                        ModuleSelectionItem::new(*module_kind, &state).render()
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
                    Rc::new(Some(clone!(state => move || {
                        html!("button-rect", {
                            .prop("kind", "text")
                            .prop("color", "lightBlue")
                            .text(STR_SHOW_ONBOARDING)
                            .event(clone!(state => move |_evt: events::Click| {
                                state.asset_edit_state.show_onboarding.set_neq(true);
                            }))
                        })
                    })))
                )
            )
            .child_signal(state.drag.signal_ref(clone!(state => move|drag| {
                drag.as_ref().map(clone!(state => move |drag| {
                    html!("img-ui", {
                        .prop("slot", "dragged")
                        .prop("path", &format!("entry/jig/modules/large/{}-hover.svg", drag.data.as_str()))
                        .style_signal("transform", drag.transform_signal())
                        .global_event(clone!(state, drag => move |evt: events::PointerMove| {
                            state.on_pointer_move(&drag, evt.x(), evt.y());
                        }))
                        .global_event(clone!(state, drag => move |evt: events::PointerUp| {
                            state.on_pointer_up(&drag, evt.x(), evt.y());
                        }))
                        .global_event(clone!(state => move |_:events::PointerCancel| {
                            state.stop_drag();
                        }))
                    })
                }))
            })))
        })
    }
}
