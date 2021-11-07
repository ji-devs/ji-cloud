use dominator::class;
use once_cell::sync::Lazy;

pub static FULL_STAGE: Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100%")
        .style("height", "100%")
    }
});


pub static SVG_FILL_TRANSPARENT_CLICK_CLASS: Lazy<String> = Lazy::new(|| {
    class! {
        .style("fill", "black")
        .style("fill-opacity", "0")
        .style("cursor", "pointer")
    }
});