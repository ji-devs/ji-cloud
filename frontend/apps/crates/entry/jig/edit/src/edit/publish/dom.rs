use dominator::{clone, html, with_node, Dom};
use futures_signals::{map_ref, signal::SignalExt};
use shared::domain::jig::JigId;
use utils::events;
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use super::{
    actions,
    components::{
        additional_resources::render as AdditionalResourcesRender, age::render as AgeRender,
        categories_pills::render as CategoriesPillsRender,
        categories_select::render as CategoriesSelectRender, goal::render as GoalRender,
        language::render as LanguageRender,
    },
    state::*,
};
use components::tooltip::{
    callbacks::TooltipErrorCallbacks,
    dom::render as TooltipDom,
    state::{
        MoveStrategy, Placement, State as TooltipState, TooltipData, TooltipError, TooltipTarget,
    },
};
use std::rc::Rc;

const STR_PUBLISH_JIG: &'static str = "Publish JIG";
const STR_PUBLIC_LABEL: &'static str = "My JIG is public";
const STR_NAME_LABEL: &'static str = "JIG’s name";
const STR_DESCRIPTION_LABEL: &'static str = "Description";
const STR_MISSING_INFO_TOOLTIP: &'static str = "Please fill in the missing information.";

pub fn render(jig_id: JigId) -> Dom {
    let state = Rc::new(State::new(jig_id));
    actions::load_data(state.clone(), jig_id);

    html!("empty-fragment", {
        .property("slot", "main")
        .child(render_page(state.clone()))
        .child(html!("window-loader-block", {
            .property_signal("visible", state.loader.is_loading())
        }))
    })
}

fn render_page(state: Rc<State>) -> Dom {
    html!("jig-edit-publish", {
        .children(&mut [
            html!("img-module-screenshot", {
                .property("slot", "thumbnail")
                .property("slot", "img")
                .property("jigId", state.jig.id.0.to_string())
                .property_signal("moduleId", state.jig.modules.signal_ref(|modules| {
                    match modules.len() {
                        0 => String::new(),
                        _ => modules[0].id.0.to_string()
                    }
                }))
            }),
            html!("label", {
                .property("slot", "public")
                .text(STR_PUBLIC_LABEL)
                .child(html!("input-switch", {
                    .property_signal("enabled", state.jig.is_public.signal_cloned())
                    .event(clone!(state => move |evt: events::CustomToggle| {
                        let value = evt.value();
                        state.jig.is_public.set(value);
                    }))
                }))
            })
            ,
            html!("input-wrapper", {
                .property("slot", "name")
                .property("label", STR_NAME_LABEL)
                .property("withHebrewButtons", true)
                .property_signal("error", {
                    (map_ref! {
                        let submission_tried = state.submission_tried.signal(),
                        let value = state.jig.display_name.signal_cloned()
                            => (*submission_tried, value.clone())
                    })
                        .map(|(submission_tried, value)| {
                            submission_tried && value.is_empty()
                        })
                })
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .property_signal("value", state.jig.display_name.signal_cloned())
                        .event(clone!(state => move |_evt: events::Input| {
                            let value = elem.value();
                            state.jig.display_name.set(value);
                        }))
                    })
                }))
            }),
            html!("input-wrapper", {
                .property("slot", "description")
                .property("label", STR_DESCRIPTION_LABEL)
                .property("withHebrewButtons", true)
                .property_signal("error", {
                    (map_ref! {
                        let submission_tried = state.submission_tried.signal(),
                        let value = state.jig.description.signal_cloned()
                            => (*submission_tried, value.clone())
                    })
                        .map(|(submission_tried, value)| {
                            submission_tried && value.is_empty()
                        })
                })
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .with_node!(elem => {
                        .text_signal(state.jig.description.signal_cloned())
                        .event(clone!(state => move |_: events::Input| {
                            let value = elem.value();
                            state.jig.description.set(value);
                        }))
                    })
                }))
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
                            let data = TooltipData::Error(Rc::new(TooltipError {
                                placement: Placement::Bottom,
                                slot: None,
                                body: String::from(STR_MISSING_INFO_TOOLTIP),
                                max_width: None,
                                callbacks: TooltipErrorCallbacks::new(Some(||{}))
                            }));

                            let target = TooltipTarget::Element(elem.clone(), MoveStrategy::Track);

                            Some(TooltipDom(Rc::new(TooltipState::new(target, data))))
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
