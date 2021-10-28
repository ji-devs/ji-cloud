use dominator::{html, Dom, clone, svg};
use utils::{prelude::*, resize::{ResizeInfo, resize_info_signal}};
use std::rc::Rc;
use futures_signals::signal::{Signal, SignalExt};
use components::traces::svg::{ShapeStyle, ShapeStyleMode, ShapeStyleKind, ShapeStyleVar, SvgCallbacks, TransformSize, render_single_shape};
use shared::domain::jig::module::body::_groups::design::TraceKind;
use super::state::*;

impl Hotspot {
    pub fn render(
        &self, 
        resize_info: &ResizeInfo, 
        on_selected: impl Fn() + 'static,
        shape_style_signal: impl Signal<Item = ShapeStyle> + 'static
    ) -> Dom {
        let shape_style = ShapeStyleVar::Dynamic(shape_style_signal);

        // this depends on calc_bounds() being implemented for PathCommands
        if self.raw.transform_matrix.is_some() {
            log::error!("shape has a transform matrix, but no size")
        }

        render_single_shape(
            shape_style, 
            &resize_info, 
            &self.raw.shape, 
            TransformSize::none(), 
            SvgCallbacks::select(move || {
                on_selected();
            })
        )
    }
}