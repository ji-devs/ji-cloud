use super::state::*;
use crate::schools::VerifiedFilter;
use dominator::{clone, html, Dom};
use futures_signals::{map_ref, signal::SignalExt, signal_vec::SignalVecExt};
use shared::domain::{billing::AdminSchool, Page};
use std::rc::Rc;
use utils::{events, prelude::*, routes::AdminSchoolsRoute};
use wasm_bindgen::JsValue;
use web_sys::HtmlInputElement;

impl SchoolTable {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        state.load_data();

        html!("empty-fragment", {
            .future(state.parent.search_filters.change_signal().for_each(clone!(state => move |_| {
                state.load_data();
                async {}
            })))
            .child(html!("window-loader-block", {
                .prop("slot", "loader")
                .prop_signal("visible", state.parent.loader.is_loading())
            }))
            .child_signal(state.table_state.signal_cloned().map(clone!(state => move |table_state| {
                Some(match table_state {
                    TableState::Table => state.render_table(),
                    TableState::UploadResults(results) => state.render_upload_results(results),
                })
            })))
        })
    }

    fn render_upload_results(self: &Rc<Self>, results: Vec<String>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("h3", {
                .text("Duplicate/existing names")
            }))
            .child(html!("button", {
                .text("Back")
                .event(clone!(state => move |_e: events::Click| {
                    state.table_state.set(TableState::Table);
                }))
            }))
            .children(results.iter().map(|result| {
                html!("div", {
                    .text(result)
                })
            }))
        })
    }

    fn render_table(self: &Rc<Self>) -> Dom {
        let state = self;

        let verified_options = vec![
            VerifiedFilter::All,
            VerifiedFilter::Verified,
            VerifiedFilter::Unverified,
        ];

        html!("admin-table-school", {
            .child(html!("input-search", {
                .prop("slot", "search")
                .prop("placeholder", "Search...")
                .event(clone!(state => move |e: events::CustomSearch| {
                    state.parent.search_filters.set_query(e.query());
                }))
            }))
            .child(html!("input-select", {
                .prop("slot", "controls")
                .prop_signal("value", state.parent.search_filters.verified.signal_cloned().map(|order_by| {
                    format!("{}", order_by)
                }))
                .children(verified_options.iter().map(|option| {
                    html!("input-select-option", {
                        .text(&format!("{}", option).to_string())
                        .prop_signal("selected", state.parent.search_filters.verified.signal_cloned().map(clone!(option => move |filter| {
                            filter == option
                        })))
                        .event(clone!(state, option => move |evt: events::CustomSelectedChange| {
                            if evt.selected() {
                                state.parent.search_filters.set_verified(option.clone());
                            }
                        }))
                    })
                }))
            }))
            .child(html!("table-pagination-jig", {
                .prop("slot", "controls")
                .child(html!("fa-button", {
                    .prop("slot", "back")
                    .prop("title", "Previous")
                    .prop("icon", "fa-solid fa-chevron-left")
                    .prop_signal("disabled", state.parent.search_filters.active_page.signal().map(|active_page| {
                        active_page == 0.into()
                    }))
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.parent.search_filters.active_page.get();
                        state.go_to_page(active_page.prev_page());
                    }))
                }))
                .child(html!("fa-button", {
                    .prop("slot", "next")
                    .prop("title", "Next")
                    .prop("icon", "fa-solid fa-chevron-right")
                    .prop_signal("disabled", map_ref! {
                        let total_pages = state.total_pages.signal(),
                        let active_page = state.parent.search_filters.active_page.signal() => {
                            match total_pages {
                                None => true,
                                Some(total_pages) => {
                                    // active_page is 0 indexed in the code side, so need to add 1 for display
                                    usize::from(*active_page) == usize::from(*total_pages).saturating_sub(1)
                                }
                            }
                        }
                    })
                    .event(clone!(state => move |_: events::Click| {
                        let active_page = state.parent.search_filters.active_page.get();
                        state.go_to_page(active_page.next_page());
                    }))
                }))
                .child_signal(state.total_pages.signal().map(clone!(state => move |total_pages| {
                    total_pages.map(|total_pages| {
                        html!("input-select", {
                            .style("width", "150px")
                            .prop_signal("value", state.parent.search_filters.active_page.signal().map(|active_page| {
                                format!("{}", usize::from(active_page) + 1)
                            }))
                            .children((0..usize::from(total_pages)).map(|page| {
                                let page = Page::from(page);
                                html!("input-select-option", {
                                    .text(&format!("{}", page.next_page()).to_string())
                                    .prop_signal("selected", state.parent.search_filters.active_page.signal().map(clone!(page => move |active_page| {
                                        page == active_page
                                    })))
                                    .event(clone!(state, page => move |evt: events::CustomSelectedChange| {
                                        if evt.selected() {
                                            state.go_to_page(page);
                                        }
                                    }))
                                })
                            }))
                        })
                    })
                })))
            }))
            .child(html!("input-wrapper", {
                .prop("slot", "controls")
                .prop("label", "Import schools")
                .child(html!("input", {
                    .prop("type", "file")
                    .prop_signal("disabled", state.uploading.signal_cloned())
                    .prop("accept", "text/csv")
                    .event(clone!(state => move |evt: events::Change| {
                        let target: JsValue = evt.target().unwrap_ji().into();
                        let element: HtmlInputElement = target.into();

                        if let Some(file) = element.files().unwrap_ji().get(0) {
                            state.upload_school_import_csv(file);
                        }
                    }))
                }))
            }))
            .children_signal_vec(state.schools.signal_vec_cloned().map(clone!(state => move |school: Rc<AdminSchool>| {
                html!("admin-table-line", {
                    .child(html!("a", {
                        .prop("dir", "auto")
                        .text(&school.school_name)
                        .event(clone!(state, school => move |_: events::Click| {
                            let route = AdminSchoolsRoute::School(school.id);
                            state.parent.navigate_to(route);
                        }))
                    }))
                    .apply(clone!(school => move |dom| {
                        match school.internal_school_name.as_ref() {
                            Some(school_name) => {
                                dom.child(html!("div", {
                                    .prop("dir", "auto")
                                    .text(&school_name.name)
                                }))
                            },
                            None => {
                                dom.child(html!("div"))
                            }
                        }
                    }))
                    .child(html!("input-checkbox", {
                        .prop("checked", school.verified)
                        .prop("disabled", school.verified)
                        .event(clone!(state, school => move |evt:events::CustomToggle| {
                            if !school.verified {
                                state.set_verified(school.id, evt.value());
                            }
                        }))
                    }))
                })
            })))
        })
    }
}
