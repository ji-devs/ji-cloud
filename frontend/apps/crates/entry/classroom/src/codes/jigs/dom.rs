use std::rc::Rc;

use components::asset_card::{render_asset_card, AssetCardBottomIndicator, AssetCardConfig};
use dominator::{clone, html, Dom, DomBuilder};
use futures_signals::signal::SignalExt;
use shared::domain::{asset::Asset, jig::codes::JigWithCodes};
use utils::{
    component::Component,
    link,
    routes::{ClassroomCodesRoute, ClassroomRoute, Route},
};
use web_sys::ShadowRoot;

use super::Jigs;

impl Component<Jigs> for Rc<Jigs> {
    fn styles() -> &'static str {
        include_str!("./styles.css")
    }

    fn dom(&self, dom: DomBuilder<ShadowRoot>) -> DomBuilder<ShadowRoot> {
        let state = self;

        state.load_data();

        dom.child(html!("div", {
            .child_signal(state.jigs.signal_cloned().map(clone!(state => move |jigs| {
                Some(match jigs {
                    Some(jigs) => state.render_loaded(jigs),
                    None => html!("progress"),
                })
            })))
        }))
    }
}

impl Jigs {
    fn render_loaded(self: &Rc<Self>, jigs: Vec<JigWithCodes>) -> Dom {
        html!("div", {
            .class("jigs")
            .children(jigs.into_iter().map(|jig| {
                let codes = jig.codes;
                let jig = jig.jig;
                let asset = Asset::Jig(jig.clone());
                html!("div", {
                    .class("jig-card")
                    .child(link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodes(jig.id))), {
                        .child(render_asset_card(&asset, AssetCardConfig {
                            dense: true,
                            bottom_indicator: AssetCardBottomIndicator::Author,
                            borderless: true,
                            ..Default::default()
                        }))
                    }))
                    .child(html!("section", {
                        .class("code-section")
                        .child(html!("h4", {
                            .text("Codes")
                        }))
                        .child(html!("div", {
                            .class("codes")
                            .child(html!("div", {
                                .class("header")
                            }))
                            .children(codes.into_iter().take(5).map(|code| {
                                link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodeSession(jig.id, code.index))), {
                                    .class("code")
                                    .children(&mut [
                                        html!("div", {
                                            .class("cell")
                                            .text(&code.name.unwrap_or_default())
                                        }),
                                        html!("div", {
                                            .class("cell")
                                            .text(&code.index.to_string())
                                        }),
                                    ])
                                })
                            }))
                        }))
                        .child(link!(Route::Classroom(ClassroomRoute::Codes(ClassroomCodesRoute::JigCodes(jig.id))), {
                            .class("more-codes")
                            .text("See all")
                            .child(html!("fa-icon", {
                                .prop("icon", "fa-solid fa-angles-right")
                            }))
                        }))
                    }))
                })
            }))
        })
    }
}
