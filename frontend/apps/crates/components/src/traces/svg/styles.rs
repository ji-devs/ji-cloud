use super::state::*;
use dominator::class;
use once_cell::sync::Lazy;

impl ShapeStyle {
    pub fn classes(&self) -> Vec<&'static str> {
        let mut classes = Vec::with_capacity(3);

        match self.mode {
            ShapeStyleMode::Mask => {
                classes.push(SHAPE_MODE_MASK_CLASS.as_str());
            }
            ShapeStyleMode::Transparent => {
                classes.push(SHAPE_MODE_EMPTY_FILL_CLASS.as_str());
            }
            ShapeStyleMode::Edit(mode) => {
                classes.push(SHAPE_MODE_EMPTY_FILL_CLASS.as_str());
                match mode {
                    ShapeStyleEditMode::Draw => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_EDIT_DRAW_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_EDIT_DRAW_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_EDIT_DRAW_INCORRECT_CLASS.as_str());
                        }
                    },
                    ShapeStyleEditMode::Selected => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_EDIT_SELECTED_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_EDIT_SELECTED_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_EDIT_SELECTED_INCORRECT_CLASS.as_str());
                        }
                    },
                    ShapeStyleEditMode::Deselected => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_EDIT_DESELECTED_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_EDIT_DESELECTED_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_EDIT_DESELECTED_INCORRECT_CLASS.as_str());
                        }
                    },
                    ShapeStyleEditMode::WithoutCutout => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_EDIT_WITHOUT_CUTOUT_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_EDIT_WITHOUT_CUTOUT_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_EDIT_WITHOUT_CUTOUT_INCORRECT_CLASS.as_str());
                        }
                    },
                }
            }
            ShapeStyleMode::Play(mode) => {
                classes.push(SHAPE_MODE_EMPTY_FILL_CLASS.as_str());
                match mode {
                    ShapeStylePlayMode::Selected => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_PLAY_SELECTED_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_PLAY_SELECTED_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_PLAY_SELECTED_INCORRECT_CLASS.as_str());
                        }
                    },
                    ShapeStylePlayMode::Deselected => match self.kind {
                        ShapeStyleKind::General => {
                            classes.push(SHAPE_MODE_PLAY_DESELECTED_GENERAL_CLASS.as_str());
                        }
                        ShapeStyleKind::Correct => {
                            classes.push(SHAPE_MODE_PLAY_DESELECTED_CORRECT_CLASS.as_str());
                        }
                        ShapeStyleKind::Incorrect => {
                            classes.push(SHAPE_MODE_PLAY_DESELECTED_INCORRECT_CLASS.as_str());
                        }
                    },
                    ShapeStylePlayMode::Hint => {
                        classes.push(SHAPE_MODE_PLAY_HINT_CLASS.as_str());
                    }
                }
            }
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

pub static SVG_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
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

//Edit Draw
pub(super) static SHAPE_MODE_EDIT_DRAW_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#005aff")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});

pub(super) static SHAPE_MODE_EDIT_DRAW_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#46ba6f")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});

pub(super) static SHAPE_MODE_EDIT_DRAW_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fd7c44")
        .style("stroke-width", "8")
        .style("stroke-dasharray", "16,6")
    }
});

//Edit Selected
pub(super) static SHAPE_MODE_EDIT_SELECTED_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#005aff")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_EDIT_SELECTED_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#46ba6f")
        .style("stroke-width", "9")
    }
});

pub(super) static SHAPE_MODE_EDIT_SELECTED_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fd7c44")
        .style("stroke-width", "5")
    }
});

//Edit Deslected
pub(super) static SHAPE_MODE_EDIT_DESELECTED_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#2343A0")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_EDIT_DESELECTED_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#518973")
        .style("stroke-width", "9")
    }
});

pub(super) static SHAPE_MODE_EDIT_DESELECTED_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#af6c27")
        .style("stroke-width", "5")
    }
});

//Edit Without Cutout
pub(super) static SHAPE_MODE_EDIT_WITHOUT_CUTOUT_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#AFCBF4")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_EDIT_WITHOUT_CUTOUT_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#9cddb2")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_EDIT_WITHOUT_CUTOUT_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fea559")
        .style("stroke-width", "8")
    }
});

//Play Selected
pub(super) static SHAPE_MODE_PLAY_SELECTED_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#005aff")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_PLAY_SELECTED_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#46ba6f")
        .style("stroke-width", "9")
    }
});

pub(super) static SHAPE_MODE_PLAY_SELECTED_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#fd7c44")
        .style("stroke-width", "5")
    }
});

//Play Deslected
pub(super) static SHAPE_MODE_PLAY_DESELECTED_GENERAL_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#2343A0")
        .style("stroke-width", "8")
    }
});

pub(super) static SHAPE_MODE_PLAY_DESELECTED_CORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#518973")
        .style("stroke-width", "9")
    }
});

pub(super) static SHAPE_MODE_PLAY_DESELECTED_INCORRECT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#af6c27")
        .style("stroke-width", "5")
    }
});

//Play Hint
pub(super) static SHAPE_MODE_PLAY_HINT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#AFCBF4")
        .style("stroke-width", "8")
    }
});
