use super::super::state::ModuleEnding;
use super::state::*;
use dominator::{clone, html, Dom};
use futures_signals::signal::SignalExt;
use std::rc::Rc;
use utils::iframe::*;

use gloo_timers::future::TimeoutFuture;

impl Ending {
    pub fn render(state: Rc<Self>) -> Dom {
        log::info!("MODULE ENDED");
        let msg = IframeAction::new(ModuleToJigPlayerMessage::Stop);

        if let Err(_) = msg.try_post_message_to_top() {
            log::info!("Couldn't post message to top!");
        }

        if let Some(kind) = state.kind {
            match kind {
                ModuleEnding::Next => {
                    log::info!("ending is next, transitioning...");
                    let msg = IframeAction::new(ModuleToJigPlayerMessage::Next);

                    if let Err(_) = msg.try_post_message_to_top() {
                        log::info!("Couldn't post message to top... redirect!");
                    }
                }
                _ => {}
            }
        }

        html!("empty-fragment", {
            .future(state.ending_finished.signal().dedupe().for_each(clone!(state => move |finished| {
                clone!(state, finished => async move {
                    if finished {
                        match state.kind {
                            Some(_kind) => {
                                {
                                    //if ending animation finished, automatically go next?
                                }
                            },
                            None => {
                                //No ending, do nothing
                            }
                        }
                    }
                })
            })))
            .apply_if(state.kind.is_some(), clone!(state => move |dom| {
                //TODO - show ending UI and only after animation/sound set ending_finished?"
                dom
                    .future(clone!(state => async move {
                        TimeoutFuture::new(3000).await;
                        state.ending_finished.set_neq(true);
                    }))
            }))
        })
    }
}
