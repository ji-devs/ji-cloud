use super::components::table::TableComponent;
use dominator::{Dom, html, with_node, clone};
use std::rc::Rc;
use super::state::*;
use web_sys::HtmlSelectElement;
use utils::events;

pub struct LocalePage {
    pub state: Rc<State>
}

impl LocalePage {
    pub fn render() -> Dom {
        // pretty bad, ha?
        super::temp_utils::add_styles(&std::include_str!("./temp_styles.css"));

        let state: Rc<State> = Rc::new(State::new());

        html!("main", {
            .children(&mut [
                html!("select" => HtmlSelectElement, {
                    .attribute("multiple", "")
                    .with_node!(elem => {
                        .event(clone!(elem => move |_:events::Change| {
                            let selected_bundle: Bundle = elem.value();
                            super::temp_utils::log(&selected_bundle);
                        }))
                    })
                    .children(
                        state.bundles.iter().map(|(e, selected)| {
                            html!("option", {
                                .property("text", e.to_string())
                                .property("value", e.to_string())
                                .property("selected", selected.clone())
                            })
                        })
                    )
                }),
                html!("div", {
                    .class("icon-button")
                    .class("select-columns")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/select-columns-icon.png")
                            }))
                            .event(clone!(state => move |_event: events::Click| {
                                state.dialog_ref
                                    .lock_ref().clone().expect("Can't get dialog")
                                    .show_modal().expect("Can't open dialog");
                            }))
                        }),
                        html!("span", {
                            .text("Select columns to display")
                        }),
                    ])
                }),
                html!("div", {
                    .class("icon-button")
                    .class("add-text")
                    .children(&mut [
                        html!("button", {
                            .child(html!("img", {
                                .attribute("src", "assets/add-icon.png")
                            }))
                            .event(clone!(state => move |_event: events::Click| {
                                state.loader.load(clone!(state => async move {
                                    state.add_translation().await;
                                }))
                            }))
                        }),
                        html!("span", {
                            .text("Add a text")
                        }),
                    ])
                }),
                TableComponent::render(state),
            ])
        })
    }
}
