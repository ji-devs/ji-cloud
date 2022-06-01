use dominator::{clone, html, with_node, Dom};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::SignalVecExt,
};
use shared::domain::asset::PrivacyLevel;
use utils::{
    events,
    routes::{AssetEditRoute, AssetRoute, CourseEditRoute, JigEditRoute, Route},
};
use web_sys::{HtmlElement, HtmlInputElement, HtmlTextAreaElement};

use super::add_additional_resource::AddAdditionalResource;
use super::additional_resource::AdditionalResourceComponent;

use super::state::Publish;
use super::{super::state::State as JigEditState, editable_assets::EditableAsset};
use components::{
    hebrew_buttons::HebrewButtons,
    module::_common::thumbnail::{ModuleThumbnail, ThumbnailFallback},
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

pub mod age;
pub mod categories_select;
pub mod category_pills;
pub mod language;

const STR_PUBLISH: &str = "Publish ";
const STR_PUBLISH_LATER: &str = "Publish later";
const STR_PUBLIC_LABEL_1: &str = "My ";
const STR_PUBLIC_LABEL_2: &str = " is public";
const STR_NAME_LABEL: &str = "’s name";
const STR_NAME_PLACEHOLDER_1: &str = "Type your ";
const STR_NAME_PLACEHOLDER_2: &str = "’s name here";
const STR_DESCRIPTION_LABEL: &str = "Description";
const STR_DESCRIPTION_PLACEHOLDER_1: &str = "What's this ";
const STR_DESCRIPTION_PLACEHOLDER_2: &str = " about?";
const STR_PUBLIC_POPUP_TITLE: &str = "Sharing is Caring!";
const STR_PUBLIC_POPUP_BODY_1: &str = "Are you sure you want to keep this ";
const STR_PUBLIC_POPUP_BODY_2: &str = " private? Please consider sharing your ";
const STR_PUBLIC_POPUP_BODY_3: &str = " with the Jigzi community.";
const STR_MISSING_INFO_TOOLTIP: &str = "Please fill in the missing information.";

impl Publish {
    pub fn render(jig_edit_state: Rc<JigEditState>) -> Dom {
        let state: Mutable<Option<Rc<Publish>>> = Mutable::new(None);

        html!("empty-fragment", {
            .future(clone!(state => async move {
                let _state = Publish::load_new(jig_edit_state).await;
                state.set(Some(Rc::new(_state)));
            }))
            .property("slot", "main")
            .child_signal(state.signal_cloned().map(|state| {
                state.map(render_page)
            }))
            .child(html!("window-loader-block", {
                .property_signal("visible", state.signal_ref(|state| state.is_none()))
            }))
        })
    }
}

fn render_page(state: Rc<Publish>) -> Dom {
    html!("jig-edit-publish", {
        .property("assetDisplayName", state.asset_type_name())
        .property("resourceOnTop", state.asset.is_resource())
        // .apply_if(state.jig.jig_focus.is_resources(), |dom| {
        //     // TODO set content for no activities and content for incomplete activities.
        //     if !has_modules {
        //         // TODO
        //     } else if let Some(_invalid_module) = invalid_module {
        //         // TODO
        //     }
        //     dom
        // })
        .children(&mut [
            ModuleThumbnail::render_live(
                Rc::new(ModuleThumbnail {
                    asset_id: state.asset.id(),
                    module: state.asset.cover().clone(),
                    fallback: ThumbnailFallback::Asset,
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
                    .text(STR_PUBLIC_LABEL_1)
                    .text(state.asset_type_name())
                    .text(STR_PUBLIC_LABEL_2)
                    .child(html!("input-switch", {
                        .property_signal("enabled", state.asset.privacy_level().signal().map(|privacy_level| {
                            privacy_level == PrivacyLevel::Public
                        }))
                        .event(clone!(state => move |evt: events::CustomToggle| {
                            let value = evt.value();
                            if value {
                                state.asset.privacy_level().set(PrivacyLevel::Public);
                                state.show_public_popup.set(false);
                            } else {
                                state.asset.privacy_level().set(PrivacyLevel::Unlisted);
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
                                    .property("body", format!(
                                        "{}{}{}{}{}",
                                        STR_PUBLIC_POPUP_BODY_1,
                                        state.asset_type_name(),
                                        STR_PUBLIC_POPUP_BODY_2,
                                        state.asset_type_name(),
                                        STR_PUBLIC_POPUP_BODY_3
                                    ))
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
                .property("label", format!("{}{}",  state.asset_type_name(), STR_NAME_LABEL))
                .property_signal("error", {
                    (map_ref! {
                        let submission_tried = state.submission_tried.signal(),
                        let value = state.asset.display_name().signal_cloned()
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
                        .property("placeholder", format!("{}{}{}", STR_NAME_PLACEHOLDER_1, state.asset_type_name(), STR_NAME_PLACEHOLDER_2))
                        .property_signal("value", state.asset.display_name().signal_cloned())
                        .event(clone!(state => move |_evt: events::Input| {
                            let value = elem.value();
                            state.asset.display_name().set(value);
                        }))
                    })
                }))
            }),
            html!("input-wrapper", {
                .property("slot", "description")
                .property("label", STR_DESCRIPTION_LABEL)
                // .property_signal("error", {
                //     (map_ref! {
                //         let submission_tried = state.submission_tried.signal(),
                //         let value = state.asset.description().signal_cloned()
                //             => (*submission_tried, value.clone())
                //     })
                //         .map(|(submission_tried, value)| {
                //             submission_tried && value.is_empty()
                //         })
                // })
                .child({
                    HebrewButtons::short().render(Some("hebrew-inputs"))
                })
                .child(html!("textarea" => HtmlTextAreaElement, {
                    .with_node!(elem => {
                        .property("placeholder", format!(
                            "{}{}{}",
                            STR_DESCRIPTION_PLACEHOLDER_1,
                            state.asset_type_name(),
                            STR_DESCRIPTION_PLACEHOLDER_2
                        ))
                        .text_signal(state.asset.description().signal_cloned())
                        .event(clone!(state => move |_: events::Input| {
                            let value = elem.value();
                            state.asset.description().set(value);
                        }))
                    })
                }))
            }),

            Publish::render_ages(state.clone()),
            Publish::render_languages(state.clone()),
            Publish::render_categories_select(state.clone()),
            Publish::render_category_pills(state.clone()),

            html!("button-rect", {
                .property("slot", "publish-later")
                .property("color", "blue")
                .property("kind", "text")
                .text(STR_PUBLISH_LATER)
                .event(clone!(state => move |_: events::Click| {
                    let url = match &state.asset {
                        EditableAsset::Jig(jig) => {
                            state.jig_edit_state.set_route_jig(JigEditRoute::Landing);
                            Route::Asset(AssetRoute::Edit(AssetEditRoute::Jig(
                                jig.id,
                                jig.jig_focus,
                                JigEditRoute::Landing
                            ))).to_string()
                        },
                        EditableAsset::Course(course) => {
                            state.jig_edit_state.set_route_jig(JigEditRoute::Landing);
                            Route::Asset(AssetRoute::Edit(AssetEditRoute::Course(
                                course.id,
                                CourseEditRoute::Landing
                            ))).to_string()
                        },
                    };
                    dominator::routing::go_to_url(&url);
                }))
            }),

            html!("div" => HtmlElement, {
                .property("slot", "publish")
                .with_node!(elem => {
                    .child(html!("button-rect", {
                        .text(STR_PUBLISH)
                        .text(state.asset_type_name())
                        .property("disabled", state.is_ready_to_publish())
                        .child(html!("fa-icon", {
                            .property("icon", "fa-light fa-rocket-launch")
                            .style("color", "var(--main-yellow)")
                        }))
                        .event(clone!(state => move |_: events::Click| {
                            Rc::clone(&state).save_asset();
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
        .apply_if(!state.asset.is_resource(), clone!(state => move |dom|{
            dom
                .children_signal_vec(state.asset.additional_resources().signal_vec_cloned().map(clone!(state => move |additional_resource| {
                    AdditionalResourceComponent::new(
                        additional_resource,
                        Rc::clone(&state)
                    ).render()
                })))
                .child(AddAdditionalResource::new(Rc::clone(&state)).render())
        }))
        .apply_if(state.asset.is_resource(), |dom|{
            dom.child_signal(state.asset.additional_resources().signal_vec_cloned().len().map(clone!(state => move|len| {
                if len == 0 {
                    Some(AddAdditionalResource::new(Rc::clone(&state)).render())
                } else {
                    let resource = state.asset.additional_resources().lock_ref()[0].clone();
                    Some(
                        AdditionalResourceComponent::new(
                            resource,
                            Rc::clone(&state)
                        ).render()
                    )
                }
            })))
        })
    })
}
