use dominator::{html, Dom, clone};
use super::state::*;
use std::rc::Rc;
use super::super::state::ModuleEnding;
use utils::{prelude::*, iframe::*};
use futures_signals::signal::SignalExt;

impl Ending {
    pub fn render(state: Rc<Self>) -> Dom {

        html!("empty-fragment", {
            .future(state.ending_finished.signal().dedupe().for_each(clone!(state => move |finished| {
                clone!(state, finished => async move {
                    if finished {
                        log::info!("finished!");
                        match state.kind {
                            //TODO - if Some, *don't* tell the player, they must hit "next" ?
                            _ => {
                                let msg = IframeAction::new(ModuleToJigPlayerMessage::Next);

                                if let Err(_) = msg.try_post_message_to_top() {
                                    log::info!("Couldn't post message to top... redirect!");
                                }
                            }
                        }
                    }
                })
            })))
            .apply_if(state.kind.is_some(), clone!(state => move |dom| {
                log::info!("TODO - show ending UI and only after animation/sound set ending_finished?");

                state.ending_finished.set_neq(true);

                dom
            }))
        })
    }
}