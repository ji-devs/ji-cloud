use once_cell::sync::Lazy;
use dominator::class;

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

pub(super) static SHAPE_MODE_TRANSPARENT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill-opacity", "0")
    }
});

pub(super) static SHAPE_MODE_DEFAULT_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill-opacity", "0")
    }
});
//Wrong mode
pub(super) static SHAPE_STATE_KIND_WRONG_DRAWING_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_WRONG_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_WRONG_DESELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_WRONG_OUTLINE_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});

//Correct mode
pub(super) static SHAPE_STATE_KIND_CORRECT_DRAWING_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_CORRECT_SELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_CORRECT_DESELECTED_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
    }
});
pub(super) static SHAPE_STATE_KIND_CORRECT_OUTLINE_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
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
pub(super) static SHAPE_STATE_KIND_REGULAR_OUTLINE_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("stroke", "#AFCBF4")
        .style("stroke-width", "8")
    }
});

