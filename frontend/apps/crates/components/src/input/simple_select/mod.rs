mod dom;
mod state;

pub use dom::*;
pub use state::*;

pub trait Item: Clone {
    fn value(&self) -> &str;
    fn label(&self) -> &str;
}

impl Item for &str {
    fn value(&self) -> &str {
        self
    }

    fn label(&self) -> &str {
        self
    }
}
