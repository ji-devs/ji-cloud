use dominator::{Dom, clone, html, with_node};
use futures_signals::{map_ref, signal::SignalExt};
use shared::domain::jig::JigId;
use utils::events;
use web_sys::HtmlElement;

use super::{
    components::{
        categories_select::render as CategoriesSelectRender,
        categories_pills::render as CategoriesPillsRender,
        goal::render as GoalRender,
        age::render as AgeRender,
        language::render as LanguageRender,
        additional_resources::render as AdditionalResourcesRender,
    },
    actions,
    state::*
};
use std::rc::Rc;
use components::tooltip::{
    dom::TooltipDom,
    types::{TooltipData, TooltipError, MoveStrategy, Placement},
};

const STR_PUBLISH_JIG: &'static str = "Publish JIG";
const STR_PUBLIC_LABEL: &'static str = "My JIG is public";
const STR_NAME_LABEL: &'static str = "JIG’s name";
const STR_DESCRIPTION_LABEL: &'static str = "Description";
const STR_MISSING_INFO_TOOLTIP: &'static str = "Please fill in the missing information.";



pub fn render(jig_id: JigId) -> Dom {
    let state = Rc::new(State::new(jig_id));
    actions::load_data(state.clone(), jig_id);

    html!("empty-fragment", {
        .child(render_page(state.clone()))
        .child(html!("window-loader-block", {
            .property_signal("visible", state.loader.is_loading())
        }))
    })
}


fn render_page(state: Rc<State>) -> Dom {
    html!("creator-publish", {
        .children(&mut [
            html!("img-ji", {
                .property("slot", "img")
                .property("lib", "mock")
                .property("size", "full")
                .property("id", "jig-gallery.jpg")
            }),
            html!("input-switch", {
                .property("slot", "public")
                .property("label", STR_PUBLIC_LABEL)
            }),
            html!("input-text", {
                .property("slot", "name")
                .property("label", STR_NAME_LABEL)
                .property_signal("value", state.jig.display_name.signal_cloned().map(|v| match v {
                    Some(v) => v,
                    None => String::new(),
                }))
                .property_signal("error", {
                    (map_ref! {
                        let submission_tried = state.submission_tried.signal(),
                        let value = state.jig.display_name.signal_cloned()
                            => (*submission_tried, value.clone())
                    })
                        .map(|(submission_tried, value)| {
                            if submission_tried && value.is_none() {
                                String::from(" ")
                            } else {
                                String::new()
                            }
                        })
                })
                .event(clone!(state => move |evt: events::CustomChange| {
                    let value = evt.value();
                    let value = if value.is_empty() {
                        None
                    } else {
                        Some(value)
                    };
                    state.jig.display_name.set(value);
                }))
            }),
            html!("input-form-textarea", {
                .property("slot", "description")
                .property("label", STR_DESCRIPTION_LABEL)
                .property("value", "This game is about… using … Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry’s standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum.")
            }),

            AgeRender(state.clone()),
            LanguageRender(state.clone()),
            GoalRender(state.clone()),
            CategoriesSelectRender(state.clone()),
            CategoriesPillsRender(state.clone()),

            html!("div" => HtmlElement, {
                .property("slot", "publish")
                .with_node!(elem => {
                    .child(html!("button-rect", {
                        .property("iconAfter", "rocket")
                        .text(STR_PUBLISH_JIG)
                        .event(clone!(state => move |_: events::Click| {
                            actions::save_jig(state.clone());
                        }))
                    }))
                    .child_signal(state.submission_tried.signal().map(clone!(elem => move |submission_tried| {
                        if submission_tried {
                            Some(TooltipDom::render(TooltipData::Error(TooltipError {
                                elem: elem.clone(),
                                placement: Placement::Bottom,
                                slot: None,
                                body: String::from(STR_MISSING_INFO_TOOLTIP),
                                max_width: None,
                                on_close: None,
                                move_strategy: MoveStrategy::Track,
                            })))
                        } else {
                            None
                        }
                    })))
                })
            }),
        ])
        .children(AdditionalResourcesRender(state.clone()))
    })
}
