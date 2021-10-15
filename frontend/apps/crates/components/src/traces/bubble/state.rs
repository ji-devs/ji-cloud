use crate::animation::fade::*;
use crate::audio::mixer::AudioHandle;
use crate::tooltip::state::{
    Anchor, ContentAnchor, State as TooltipState, TooltipBubble, TooltipData, TooltipTarget,
};
use crate::tooltip::state::{MoveStrategy, TooltipContainer};
use dominator::clone;
use futures_signals::signal::Mutable;
use shared::domain::jig::module::body::Audio;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use utils::math::bounds::BoundsF64;

pub struct TraceBubble {
    pub audio: Option<Audio>,
    pub fade: Fade,
    pub end_policy: Cell<EndPolicy>,
    pub(super) audio_handle: RefCell<Option<AudioHandle>>,
    pub(super) tooltip: Option<Rc<TooltipState>>,
    /// Will only fire when both audio and fade have ended
    pub(super) on_ended: Option<Box<dyn Fn()>>,
    pub(super) fade_ended: AtomicBool,
    pub(super) audio_ended: AtomicBool,
    pub(super) dispatched_ended: AtomicBool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndPolicy {
    Any,
    All,
    //If audio exists, ends when that finishes. Otherwise fade
    AudioThenFade,
}

impl TraceBubble {
    pub fn new(
        bounds: BoundsF64,
        audio: Option<Audio>,
        text: Option<String>,
        on_ended: Option<impl Fn() + 'static>,
    ) -> Rc<Self> {
        let tooltip = text.map(|text| {
            let mut state = TooltipState::new(
                TooltipTarget::NormalizedBounds(bounds, MoveStrategy::Track),
                TooltipData::Bubble(Rc::new(TooltipBubble {
                    target_anchor: Anchor::Bottom,
                    content_anchor: ContentAnchor::OppositeV,
                    body: text,
                    max_width: Some(200.0),
                })),
            );

            state.container = Some(TooltipContainer::MainOrWindow);

            Rc::new(state)
        });

        let _self_ref: Rc<RefCell<Option<Rc<Self>>>> = Rc::new(RefCell::new(None));

        let _self = Rc::new(Self {
            audio,
            audio_handle: RefCell::new(None),
            tooltip,
            end_policy: Cell::new(EndPolicy::AudioThenFade),
            fade: Fade::new(
                FadeKind::Out,
                6_000.0,
                true,
                None,
                Some(clone!(_self_ref => move || {
                    if let Some(_self) = _self_ref.borrow().as_ref() {
                        _self.on_fade_ended();
                    }
                })),
            ),
            on_ended: on_ended.map(|f| Box::new(f) as _),
            fade_ended: AtomicBool::new(false),
            audio_ended: AtomicBool::new(false),
            dispatched_ended: AtomicBool::new(false),
        });

        *_self_ref.borrow_mut() = Some(_self.clone());

        _self
    }

    //Will manage its own lifetime by way of a specific Mutable type
    pub fn set_unset_mutable(
        bounds: BoundsF64,
        audio: Option<Audio>,
        text: Option<String>,
        mutable: Mutable<Option<Rc<TraceBubble>>>,
    ) {
        let instance = TraceBubble::new(
            bounds,
            audio,
            text,
            Some(clone!(mutable => move || {
                mutable.set(None)
            })),
        );

        mutable.set(Some(instance));
    }
}
