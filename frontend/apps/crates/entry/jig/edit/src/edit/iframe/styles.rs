use dominator::class;
use once_cell::sync::Lazy;

pub static IFRAME_CLASS: Lazy<String> = Lazy::new(|| {
  class! {
      .style("border", "none")
  }
});