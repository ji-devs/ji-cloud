use dominator::{
    DomBuilder,
    clone,
    animation::{easing, Percentage, MutableAnimation, AnimatedMapBroadcaster}
};
use futures_signals::signal::{Signal, SignalExt};
use web_sys::{Element, HtmlElement};
use utils::resize::{ResizeInfo, get_resize_info};
use std::rc::Rc;
use gloo_timers::future::TimeoutFuture;

#[derive(Clone, Copy, Debug)]
pub enum FadeKind {
    In,
    Out
}

pub struct Fade {
    kind: FadeKind,
    animation: Rc<MutableAnimation>,
    hide_on_finished: bool,
    delay: Option<f64>,
}

impl Fade {
    pub fn new(kind: FadeKind, duration: f64, hide_on_finished: bool, delay: Option<f64>) -> Self {
        Self {
            kind,
            animation: Rc::new(MutableAnimation::new(duration)),
            hide_on_finished,
            delay
        }
    }

    pub fn render<A>(&self, dom: DomBuilder<A>) -> DomBuilder<A> 
    where
        A: AsRef<HtmlElement> + AsRef<Element>
    {

        let Self {kind, animation, hide_on_finished, delay} = self;

        let value_signal = animation.signal()
            //TODO support configurable easing
            .map(move |t| easing::in_out(t, easing::cubic))
            .map(clone!(kind => move |t| {
                let value = t.into_f64();
                match kind {
                    FadeKind::Out => 1.0 - value,
                    FadeKind::In => value
                }
            }));

        let visible_signal = animation.signal() 
            .map(|value| value != Percentage::new(1.0));

        dom
            .future(clone!(animation, delay => async move {
                if let Some(delay) = delay {
                    TimeoutFuture::new(delay as u32).await;
                }
                animation.animate_to(Percentage::new(1.0));
            }))
            .style_signal("opacity", 
                value_signal
                .map(|value| {
                    format!("{}", value)
                })
            )
            .apply_if(self.hide_on_finished, |dom| {
                dom.visible_signal(visible_signal)
            })
    }
}

