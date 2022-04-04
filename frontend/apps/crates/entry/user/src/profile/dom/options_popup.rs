use std::rc::Rc;

use dominator::{clone, html, Dom};
use futures_signals::{
    signal::SignalExt,
    signal_vec::{MutableVec, SignalVecExt},
};
use shared::domain::meta::MetadataResponse;
use utils::events;

use crate::profile::{actions, state::ActivePopup};

use super::super::state::{ProfilePageUser, State};

const STR_DONE: &str = "Done";

pub struct PopupCallbacks<I, S> {
    pub get_options_list: Box<dyn Fn(&MetadataResponse) -> &Vec<S>>,
    pub get_selected_list: Box<dyn Fn(&ProfilePageUser) -> &MutableVec<I>>,
    pub get_id_from_struct: Box<dyn Fn(&S) -> &I>,
    pub get_display_name: Box<dyn Fn(&S) -> &str>,
}

pub fn render<I, S>(
    state: Rc<State>,
    header: &str,
    subheader: &str,
    callbacks: PopupCallbacks<I, S>,
) -> Dom
where
    I: 'static + Clone + PartialEq,
    S: 'static,
{
    let callbacks = Rc::new(callbacks);
    html!("user-profile-options-popup", {
        .property("header", header)
        .property("subheader", subheader)
        .children(&mut [
            html!("fa-button", {
                .property("slot", "close")
                .property("icon", "fa-light fa-xmark")
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                    actions::save_profile(Rc::clone(&state));
                }))
            }),
            html!("button-rect", {
                .property("slot", "done")
                .property("color", "blue")
                .text(STR_DONE)
                .event(clone!(state => move |_: events::Click| {
                    state.active_popup.set(ActivePopup::None);
                }))
            }),
        ])
        .children_signal_vec(state.metadata.signal_ref(clone!(state => move |metadata| {
            match metadata {
                None => vec![],
                Some(metadata) => {
                    let options = (callbacks.get_options_list)(metadata);

                    options.iter().map(clone!(state, callbacks => move |age| {
                        let age_id = (callbacks.get_id_from_struct)(age).clone();
                        html!("input-checkbox", {
                            .property("slot", "options")
                            .property("label", &*(callbacks.get_display_name)(age))
                            .property_signal("checked", (callbacks.get_selected_list)(&state.user).signal_vec_cloned().to_signal_cloned().map(clone!(age_id => move |ages| {
                                ages.contains(&age_id)
                            })))
                            .event(clone!(state, callbacks => move |_: events::CustomToggle| {
                                let mut age_ranges = (callbacks.get_selected_list)(&state.user).lock_mut();

                                match age_ranges.iter().position(|age| {
                                    age == &age_id
                                }) {
                                    Some(pos) => {
                                        age_ranges.remove(pos);
                                    },
                                    None => {
                                        age_ranges.push_cloned(age_id.clone());
                                    },
                                };

                                actions::save_profile(Rc::clone(&state));
                            }))
                        })
                    })).collect::<Vec<Dom>>()
                },
            }
        })).to_signal_vec())
    })
}
