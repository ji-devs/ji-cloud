
use std::rc::Rc;
use super::state::*;
use dominator::{Dom, html, svg, clone, EventOptions};
use utils::{prelude::*, resize::{ResizeInfo, resize_info_signal}};
use futures_signals::signal::{Signal, SignalExt, always};
use crate::base::{
    styles::FULL_STAGE,
    activities::_common::hotspot::*
};

use components::traces::svg::{ShapeStyle, ShapeStyleMode, ShapeStyleState};
use shared::domain::jig::module::body::_groups::design::TraceKind;

impl Soundboard {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        svg!("svg", {
            .class(&*FULL_STAGE)
            .children_signal_vec(
                resize_info_signal().map(clone!(state => move |resize_info| {
                    state.items
                        .iter()
                        .map(|item| item.clone().render_svg_path(state.clone(), &resize_info))
                        .collect()
                }))
                .to_signal_vec()
            )
        })
    }
}

impl SoundboardItem {
    pub fn render_svg_path(self: Rc<Self>, parent: Rc<Soundboard>, resize_info: &ResizeInfo) -> Dom {
        let state = self;
        state.hotspot.render(
            &resize_info,
            clone!(state, parent => move || {
                let was_revealed = state.revealed.replace(true);
                if !was_revealed {
                    log::info!("first time!");
                }
            }),
            state.revealed.signal().map(|revealed| {
                ShapeStyle {
                    interactive: true,
                    mode: if revealed { Some(ShapeStyleMode::Solid) } else { Some(ShapeStyleMode::Transparent) },
                    kind: Some(TraceKind::Regular),
                    state: Some(ShapeStyleState::Deselected),
                }
            })
        )
    }
}