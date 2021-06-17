use futures_signals::signal::{Mutable, SignalExt, from_stream};
use gloo::timers::future::IntervalStream;
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug)]
pub struct Timer {
    pub time: Mutable<u32>,
}

impl Timer {
    pub fn new(seconds: u32) -> Self {
        let _self = Self {
            time: Mutable::new(seconds + 1),
        };
        _self.init();
        _self
    }

    fn init(&self) {
        let m = self.time.clone();
        let signal = from_stream(IntervalStream::new(1_000)).map(move |_| {
            let mut time = m.lock_mut();
            if *time > 0 {
                *time = *time - 1;
            }
        });

        let future = signal.for_each(|_| {
            async {}
        });

        spawn_local(future);
    }
}

// TODO: impl drop, signal
