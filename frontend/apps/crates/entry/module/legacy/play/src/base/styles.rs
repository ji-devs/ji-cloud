use dominator::class;
use once_cell::sync::Lazy;

pub const FULL_STAGE: Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100%")
        .style("height", "100%")
    }
});
