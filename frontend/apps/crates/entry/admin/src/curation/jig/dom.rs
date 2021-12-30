use super::state::*;
use dominator::{html, Dom, clone};
use utils::{events, routes::AdminCurationRoute};
use std::rc::Rc;

impl CurationJig {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("admin-curation-jig-details", {
            .attribute("slot", "jig-details")
            .children(&mut [
                html!("button-rect", {
                    .property("slot", "back")
                    .property("color", "blue")
                    .property("kind", "text")
                    .text("Back")
                    .event(clone!(state => move |_: events::Click| {
                        let route = AdminCurationRoute::Table;
                        state.curation_state.navigate_to(route);
                    }))
                }),
                html!("div", {
                    .attribute("slot", "buttons")
                    .children(&mut [
                        html!("button-rect", {
                            .attribute("kind", "text")
                            .attribute("color", "blue")
                            .text("JIG's name")
                        }),
                        html!("button-rect", {
                            .attribute("kind", "outline")
                            .attribute("color", "blue")
                            .text("Save Changes")
                        }),
                    ])
                }),
                html!("div", {
                    .attribute("slot", "inputs")
                    .children(&mut [
                        html!("input-wrapper", {
                            .attribute("label", "JIG's name")
                            .children(&mut [
                                html!("input", {
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "Author name")
                            .children(&mut [
                                html!("input", {
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Instruction Language")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("English")
                                }),
                                html!("input-select-option", {
                                    .text("Spanish")
                                }),
                                html!("input-select-option", {
                                    .text("Hebrew")
                                }),
                                html!("input-select-option", {
                                    .text("French")
                                }),
                                html!("input-select-option", {
                                    .text("Italian")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Suitable for age")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("All ages")
                                }),
                                html!("input-select-option", {
                                    .text("No ages")
                                }),
                            ])
                        }),
                        html!("input-select", {
                            .attribute("label", "Affiliation")
                            .children(&mut [
                                html!("input-select-option", {
                                    .text("Affiliation 1")
                                }),
                                html!("input-select-option", {
                                    .text("Affiliation 2")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "JIG teacher's description")
                            .children(&mut [
                                html!("textarea", {
                                    .attribute("id", "description")
                                    .attribute("rows", "6")
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                        html!("input-wrapper", {
                            .attribute("label", "Additional keywords")
                            .children(&mut [
                                html!("textarea", {
                                    .attribute("rows", "6")
                                    .attribute("value", "")
                                }),
                            ])
                        }),
                    ])
                })
            ])
        })
    }
}
