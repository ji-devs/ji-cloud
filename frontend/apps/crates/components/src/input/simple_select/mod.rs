mod dom;
mod state;

pub use dom::*;
pub use state::*;

pub trait SimpleSelectItem: Clone {
    fn value(&self) -> &str;
    fn label(&self) -> &str;
}

impl SimpleSelectItem for &str {
    fn value(&self) -> &str {
        self
    }

    fn label(&self) -> &str {
        self
    }
}
