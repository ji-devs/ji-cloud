use futures_signals::signal_vec::VecDiff;

use super::state::*;

pub struct Callbacks<T: AsSticker> {
    pub on_change: Option<Box<dyn Fn(&[T])>>,
    /// Callback for whenever an operation is performed which results in the indexes changing.
    pub on_index_change: Option<Box<dyn Fn(VecDiff<T>)>>,
}

impl<T: AsSticker> Callbacks<T> {
    pub fn new(on_change: Option<impl Fn(&[T]) + 'static>) -> Self {
        Self {
            on_change: on_change.map(|f| Box::new(f) as _),
            on_index_change: None,
        }
    }

    pub fn set_on_index_change(&mut self, on_index_change: impl Fn(VecDiff<T>) + 'static) {
        self.on_index_change = Some(Box::new(on_index_change));
    }
}
