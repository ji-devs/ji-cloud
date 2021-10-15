use super::state::*;
use dominator::class;
use once_cell::sync::Lazy;
use shared::domain::jig::module::body::_groups::design::TraceKind;
use utils::prelude::*;

impl ShapeStyle {
    pub fn classes(&self) -> Vec<&'static str> {
        let mut classes = Vec::with_capacity(3);

        match self.mode {
            None => classes.push(SHAPE_MODE_EMPTY_FILL_CLASS.as_str()),
            Some(mode) => {
                if mode == ShapeStyleMode::Mask {
                    classes.push(SHAPE_MODE_MASK_CLASS.as_str());
                } else {
                    classes.push(SHAPE_MODE_EMPTY_FILL_CLASS.as_str());

                    if mode == ShapeStyleMode::Solid {
                        if let Some(kind) = self.kind {
                            classes.push(match kind {
                                TraceKind::Wrong => SHAPE_MODE_SOLID_KIND_WRONG_CLASS.as_str(),
                                TraceKind::Correct => SHAPE_MODE_SOLID_KIND_CORRECT_CLASS.as_str(),
                                TraceKind::Regular => SHAPE_MODE_SOLID_KIND_REGULAR_CLASS.as_str(),
                            });
                        }
                    }
                }
            }
        }

        if self.kind.is_some() && self.state.is_some() {
            let state = self.state.unwrap_ji();

            classes.push(match self.kind.unwrap_ji() {
                TraceKind::Wrong => match state {
                    ShapeStyleState::Drawing => SHAPE_STATE_KIND_WRONG_DRAWING_CLASS.as_str(),
                    ShapeStyleState::Selected => SHAPE_STATE_KIND_WRONG_SELECTED_CLASS.as_str(),
                    ShapeStyleState::Deselected => SHAPE_STATE_KIND_WRONG_DESELECTED_CLASS.as_str(),
                },
                TraceKind::Correct => match state {
                    ShapeStyleState::Drawing => SHAPE_STATE_KIND_CORRECT_DRAWING_CLASS.as_str(),
                    ShapeStyleState::Selected => SHAPE_STATE_KIND_CORRECT_SELECTED_CLASS.as_str(),
                    ShapeStyleState::Deselected => {
                        SHAPE_STATE_KIND_CORRECT_DESELECTED_CLASS.as_str()
                    }
                },
                TraceKind::Regular => match state {
                    ShapeStyleState::Drawing => SHAPE_STATE_KIND_REGULAR_DRAWING_CLASS.as_str(),
                    ShapeStyleState::Selected => SHAPE_STATE_KIND_REGULAR_SELECTED_CLASS.as_str(),
                    ShapeStyleState::Deselected => {
                        SHAPE_STATE_KIND_REGULAR_DESELECTED_CLASS.as_str()
                    }
                },
            });
        }

        if self.interactive {
            classes.push(SHAPE_INTERACTIVE_CLASS.as_str());
        }

        classes
    }

    pub fn classes_string(&self) -> String {
        self.classes()
            .iter()
            .fold(String::new(), |acc, class_name| {
                format!("{} {}", acc, class_name)
            })
    }
}

pub(super) static SVG_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
    }
});

//defines the image we're going to cut holes in
pub(super) static BG_FILL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill", "black")
        .style("fill-opacity", "0.5")
    }
});

//by default, fill completely - no holes
pub(super) static BG_MASK_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill", "white")
    }
});

//shapes cut holes in the mask by painting them black
pub(super) static SHAPE_INTERACTIVE_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("cursor", "pointer")
    }
});

//shapes cut holes in the mask by painting them black
pub(super) static SHAPE_MODE_MASK_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill", "black")
    }
});

pub(super) static SHAPE_MODE_EMPTY_FILL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill-opacity", "0")
    }
});
//Wrong mode
pub(super) static SHAPE_STATE_KIND_WRONG_DRAWING_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fd7c44")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});
pub(super) static SHAPE_STATE_KIND_WRONG_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fd7c44")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_STATE_KIND_WRONG_DESELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#af6c27")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_MODE_SOLID_KIND_WRONG_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fea559")
        .style("stroke-width", "8")
    }
});

//Correct mode
pub(super) static SHAPE_STATE_KIND_CORRECT_DRAWING_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#46ba6f")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});
pub(super) static SHAPE_STATE_KIND_CORRECT_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#46ba6f")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_STATE_KIND_CORRECT_DESELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#518973")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_MODE_SOLID_KIND_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#9cddb2")
        .style("stroke-width", "8")
    }
});

//Regular mode
pub(super) static SHAPE_STATE_KIND_REGULAR_DRAWING_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#005aff")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});
pub(super) static SHAPE_STATE_KIND_REGULAR_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#005aff")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_STATE_KIND_REGULAR_DESELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#2343A0")
        .style("stroke-width", "8")
    }
});
pub(super) static SHAPE_MODE_SOLID_KIND_REGULAR_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#AFCBF4")
        .style("stroke-width", "8")
    }
});
