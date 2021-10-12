use dominator::{stylesheet, class};
use once_cell::sync::Lazy;

pub const BG:Lazy<String> = Lazy::new(|| {
    class! {
        .style("position", "absolute")
        .style("top", "0")
        .style("left", "0")
        .style("width", "100%")
        .style("height", "100%")
    }
});
