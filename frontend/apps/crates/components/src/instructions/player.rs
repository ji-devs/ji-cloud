use dominator::{Dom, html, clone};
use dominator::animation::MutableAnimation;
use futures_signals::signal_vec::SignalVecExt;
use std::rc::Rc;
use utils::prelude::*;
use futures_signals::signal::SignalExt;
use web_sys::HtmlElement;
use shared::domain::jig::module::body::Instructions;
use crate::animation::fade::*;

pub struct InstructionsPlayer {
    data: Instructions,
    fade: Fade,
}

impl InstructionsPlayer {
    pub fn new(data:Instructions) -> Self {
        let data = Instructions {
            text: Some("instructions here!".to_string()),
            audio: None
        };
        let animation = MutableAnimation::new(1000.0);
        Self {
            data,
            fade: Fade::new(
                FadeKind::Out,
                1000.0,
                true,
                Some(3000.0)
            )
        }
    }

    pub fn render(&self) -> Dom {
        html!("empty-fragment", {
            .apply_if(self.data.text.is_some(), |dom| {
                let text = self.data.text.as_ref().unwrap_ji();

                self.fade.render(dom.child(
                    html!("instructions-banner", {
                        .text(text)
                    })
                ))
                
            })
        })
    }

}
