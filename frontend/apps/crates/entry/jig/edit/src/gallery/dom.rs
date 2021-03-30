use dominator::{html, clone, Dom};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::jig::{JigId, ModuleId};
use utils::prelude::*;
use super::{actions, state::*};
use std::rc::Rc;

pub struct GalleryDom {
}

impl GalleryDom {
    pub fn render() -> Dom {
        let state = Rc::new(State::new());

        actions::load_jigs(state.clone());

        html!("article", {
            .child(html!("button", {
                .text("Create New")
                .event(clone!(state => move |evt:events::Click| {
                    actions::create_jig(state.clone());
                }))
            }))
            .child(html!("hr"))
            .child_signal(state.loaded_signal().map(clone!(state => move |(is_loading, n_loaded)| {
                if is_loading {
                    Some(html!("div", {.text("loading...")}))
                } else {
                    if n_loaded == 0 {
                        Some(html!("div", {.text("No jigs!")}))
                    } else {
                        Some(html!("ul", {
                            .children_signal_vec(state.jigs.signal_vec_cloned().map(clone!(state => move |(jig_id, name)| {
                                let route:String = Route::Jig(JigRoute::Edit(jig_id.clone(), None)).into();
                                html!("li", {
                                    .child(html!("a", {
                                        .property("href", route)
                                        .child(html!("span", {
                                            .text(&{
                                                let id = jig_id.0.to_string();
                                                match name {
                                                    None => id, 
                                                    Some(name) => format!("{} - {}", id, name)
                                                }
                                            })
                                        }))
                                    }))
                                    .child(html!("button", {
                                        .style("margin-left", "20px")
                                        .text("delete")
                                        .event(clone!(state, jig_id => move |evt:events::Click| {
                                            actions::delete_jig(state.clone(), jig_id);
                                        }))
                                    }))
                                })
                            })))
                        }))
                    }
                }
            })))
        })
    }
}
