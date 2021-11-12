use futures_signals::signal::{Signal, SignalExt};

pub fn width_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.0),
    })
}

pub fn height_signal(size: impl Signal<Item = Option<(f64, f64)>>) -> impl Signal<Item = String> {
    size.map(|size| match size {
        None => "0".to_string(),
        Some(size) => format!("{}rem", size.1),
    })
}
