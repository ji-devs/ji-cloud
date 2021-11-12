use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use shared::domain::jig::PrivacyLevel;
use utils::{
    events,
    routes::{JigEditRoute, JigRoute, Route},
};
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use super::super::state::State as JigEditState;
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
use components::{
    hebrew_buttons::HebrewButtons,
    module::_common::thumbnail::ModuleThumbnail,
    tooltip::{
        callbacks::TooltipErrorCallbacks,
        dom::render as TooltipDom,
        state::{
            Anchor, ContentAnchor, MoveStrategy, State as TooltipState, TooltipData, TooltipError,
            TooltipTarget,
        },
    },
};
use std::rc::Rc;

const STR_PUBLISH_JIG: &'static str = "Publish JIG";
const STR_PUBLISH_LATER: &'static str = "I will publish later";
const STR_PUBLIC_LABEL: &'static str = "My JIG is public";
const STR_NAME_LABEL: &'static str = "JIG’s name";
const STR_NAME_PLACEHOLDER: &'static str = "Type your JIG’s name here";
const STR_DESCRIPTION_LABEL: &'static str = "Description";
const STR_DESCRIPTION_PLACEHOLDER: &'static str =
    "This JIG is about… (include words that will help others find this JIG easily)";
const STR_PUBLIC_POPUP_TITLE: &'static str = "Sharing is Caring!";
const STR_PUBLIC_POPUP_BODY: &'static str = "Are you sure you want to keep this JIG private? Please consider sharing your JIG with the Jigzi community.";
const STR_MISSING_INFO_TOOLTIP: &'static str = "Please fill in the missing information.";

pub fn render(jig_edit_state: Rc<JigEditState>) -> Dom {
    let state: Mutable<Option<Rc<State>>> = Mutable::new(None);

    html!("empty-fragment", {
        .future(clone!(state => async move {
            let _state = State::load_new(jig_edit_state).await;
            state.set(Some(Rc::new(_state)));
        }))
        .property("slot", "main")
        .child_signal(state.signal_cloned().map(|state| {
            state.map(|state| render_page(state.clone()))
        }))
        .child(html!("window-loader-block", {
            .property_signal("visible", state.signal_ref(|state| state.is_none()))
        }))
    })
}

fn render_page(state: Rc<State>) -> Dom {
    html!("jig-edit-publish", {
        .children(&mut [
            ModuleThumbnail::render_live(
                Rc::new(ModuleThumbnail {
                    jig_id: state.jig.id.clone(),
                    //Cover module (first module) is guaranteed to exist
                    module: state.jig.modules.lock_ref()[0].clone(),
                    is_jig_fallback: true,
                }),
                Some("img")
            ),
            html!("fa-icon", {
                .property("icon", "fa-thin fa-pen")
                .property("slot", "edit-cover")
                .event(clone!(state => move |_: events::Click| {
                    state.navigate_to_cover();
                }))
            }),
            html!("label", {
                .with_node!(elem => {
                    .property("slot", "public")
                    .text(STR_PUBLIC_LABEL)
                    .child(html!("input-switch", {
                        .property_signal("enabled", state.jig.privacy_level.signal().map(|privacy_level| {
                            privacy_level == PrivacyLevel::Public
                        }))
                        .event(clone!(state => move |evt: events::CustomToggle| {
                            let value = evt.value();
                            if value {
                                state.jig.privacy_level.set(PrivacyLevel::Public);
                                state.show_public_popup.set(false);
                            } else {
                                state.jig.privacy_level.set(PrivacyLevel::Unlisted);
                                state.show_public_popup.set(true);
                            }
                        }))
                    }))
                    .child_signal(state.show_public_popup.signal_ref(clone!(state => move |show_public_popup| {
                        match show_public_popup {
                            false => None,
                            true => {
                                Some(html!("tooltip-info", {

                                    .property("title", STR_PUBLIC_POPUP_TITLE)
                                    .property("body", STR_PUBLIC_POPUP_BODY)
                                    .property("closeable", true)
                                    .property("target", elem.clone())
                                    .property("placement", "bottom")
                                    .event(clone!(state => move |_: events::Close| {
                                        state.show_public_popup.set(false);
                                    }))
                                }))
                            }
                        }
                    })))
                })
            }),
            html!("input-wrapper", {
                .property("slot", "name")
                .property("label", STR_NAME_LABEL)
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
                .child({
                    HebrewButtons::short().render(Some("hebrew-inputs"))
                })
                .child(html!("input" => HtmlInputElement, {
                    .with_node!(elem => {
                        .property("placeholder", STR_NAME_PLACEHOLDER)
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
                .child({
                    HebrewButtons::short().render(Some("hebrew-inputs"))
                })
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .with_node!(elem => {
                        .property("placeholder", STR_DESCRIPTION_PLACEHOLDER)
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

            html!("button-rect", {
                .property("slot", "publish-later")
                .property("color", "blue")
                .property("kind", "text")
                .text(STR_PUBLISH_LATER)
                .event(clone!(state => move |_: events::Click| {
                    state.jig_edit_state.route.set_neq(JigEditRoute::Landing);
                    let url:String = Route::Jig(JigRoute::Edit(state.jig.id.clone(), JigEditRoute::Landing)).into();
                    dominator::routing::go_to_url(&url);
                }))
            }),

            html!("div" => HtmlElement, {
                .property("slot", "publish")
                .with_node!(elem => {
                    .child(html!("button-rect", {
                        .text(STR_PUBLISH_JIG)
                        .child(html!("fa-icon", {
                            .property("icon", "fa-light fa-rocket-launch")
                            .style("color", "var(--main-yellow)")
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            actions::save_jig(state.clone());
                        }))
                    }))
                    .child_signal(state.show_missing_info_popup.signal().map(clone!(state, elem => move |show_popup| {
                        if show_popup {
                            let on_close = clone!(state => move|| {
                                state.show_missing_info_popup.set(false);
                            });
                            let data = TooltipData::Error(Rc::new(TooltipError {
                                target_anchor: Anchor::Top,
                                content_anchor: ContentAnchor::Bottom,
                                body: String::from(STR_MISSING_INFO_TOOLTIP),
                                max_width: None,
                                callbacks: TooltipErrorCallbacks::new(Some(on_close))
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
