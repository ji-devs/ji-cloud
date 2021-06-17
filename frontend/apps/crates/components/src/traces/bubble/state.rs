use utils::{prelude::*, math::bounds::BoundsF64};
use shared::domain::jig::module::body::Audio;
use crate::animation::fade::*;
use crate::audio_mixer::AudioInstance;
use crate::tooltip::state::MoveStrategy;
use crate::tooltip::state::{State as TooltipState, TooltipData, TooltipBubble, TooltipTarget, Placement};
use std::cell::RefCell;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use dominator::clone;

pub struct TraceBubble {
    pub audio: Option<Audio>,
    pub fade: Fade,
    pub(super) audio_player_instance: RefCell<Option<Rc<AudioInstance>>>,
    pub(super) tooltip: Option<Rc<TooltipState>>

}

impl TraceBubble {
    pub fn new(bounds: BoundsF64, audio: Option<Audio>, text: Option<String>, on_fade_end: Option<impl Fn() + 'static>) -> Self {

        let tooltip = text.map(|text| {
            Rc::new(TooltipState::new(
                TooltipTarget::NormalizedBounds(bounds, MoveStrategy::Track),
                TooltipData::Bubble(Rc::new(TooltipBubble {
                    placement: Placement::Bottom,
                    slot: None,
                    body: text,
                    max_width: Some(200.0) 
                }))
            ))
        });

        Self {
            audio,
            audio_player_instance: RefCell::new(None),
            tooltip,
            fade: Fade::new(
                FadeKind::Out,
                6_000.0,
                true,
                None,
                on_fade_end
            )
        }
    }

    //Will manage its own lifetime by way of a specific Mutable type
    pub fn set_unset_mutable(bounds: BoundsF64, audio: Option<Audio>, text: Option<String>, mutable: Mutable<Option<Rc<TraceBubble>>>) {

        let instance = Rc::new(TraceBubble::new(
            bounds, 
            audio, 
            text,
            Some(clone!(mutable => move || {
                mutable.set(None)
            }))
        ));

        mutable.set(Some(instance));
    }
}
