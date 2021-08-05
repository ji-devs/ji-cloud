use std::rc::Rc;
use dominator::{Dom, clone, html};
use futures_signals::{map_ref, signal::{Signal, SignalExt}};
use shared::domain::meta::Goal;
use utils::{events, unwrap::UnwrapJiExt};

use super::super::state::State;

const STR_TEACHING_GOAL_LABEL: &'static str = "Teaching Goal";
const STR_TEACHING_GOAL_PLACEHOLDER: &'static str = "Select from the list";

pub fn render(state: Rc<State>) -> Dom {
    html!("input-select", {
        .property("slot", "goal")
        .property("label", STR_TEACHING_GOAL_LABEL)
        .property("placeholder", STR_TEACHING_GOAL_PLACEHOLDER)
        .property("multiple", true)
        .property_signal("value", goal_value_signal(state.clone()))
        .property_signal("error", {
            (map_ref! {
                let submission_tried = state.submission_tried.signal(),
                let value = state.jig.goals.signal_cloned()
                    => (*submission_tried, value.clone())
            })
                .map(|(submission_tried, value)| {
                    submission_tried && value.is_empty()
                })
        })
        .children_signal_vec(state.goals.signal_cloned().map(clone!(state => move |goals| {
            match goals {
                None => vec![],
                Some(goals) => {
                    goals.iter().map(|goal| {
                        render_goal(&goal, state.clone())
                    }).collect()
                },
            }
            
        })).to_signal_vec())
    })
}

fn render_goal(goal: &Goal, state: Rc<State>) -> Dom {
    let goal_id = goal.id.clone();
    html!("input-select-option", {
        .text(&goal.display_name)
        .property_signal("selected", state.jig.goals.signal_cloned().map(clone!(goal_id => move |goals| {
            goals.contains(&goal_id)
        })))
        .event(clone!(state => move |_: events::CustomSelectedChange| {
            let mut goals = state.jig.goals.lock_mut();
            if goals.contains(&goal_id) {
                goals.remove(&goal_id);
            } else {
                goals.insert(goal_id); 
            }
        }))
    })
}

fn goal_value_signal(state: Rc<State>) -> impl Signal<Item = String> {
    map_ref! {
        let selected_goals = state.jig.goals.signal_cloned(),
        let available_goals = state.goals.signal_cloned() => {
            let mut output = vec![];
            if let Some(available_goals) = available_goals {
                selected_goals.iter().for_each(|goal_id| {
                    let goal = available_goals.iter().find(|goal| goal.id == *goal_id).unwrap_ji();
                    output.push(goal.display_name.clone());
                })
            }
            output.join(", ")
        }
        
    }
}
