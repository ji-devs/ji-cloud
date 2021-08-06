use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::{from_stream, Mutable, SignalExt};
use gloo::timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug)]
pub struct Timer {
    pub time: Mutable<u32>,
    pub paused: Rc<RefCell<bool>>,
}

impl Timer {
    pub fn new(seconds: u32) -> Self {
        let _self = Self {
            time: Mutable::new(seconds + 1),
            paused: Rc::new(RefCell::new(false)),
        };
        _self.init();
        _self
    }

    fn init(&self) {
        let time = self.time.clone();
        let paused = Rc::clone(&self.paused);
        let signal = from_stream(IntervalStream::new(1_000)).map(move |_| {
            if *paused.borrow() {
                return;
            }

            let mut time = time.lock_mut();
            if *time > 0 {
                *time = *time - 1;
            }
        });

        let future = signal.for_each(|_| async {});

        spawn_local(future);
    }
}

// TODO: impl drop, signal
