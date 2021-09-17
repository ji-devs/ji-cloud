use cfg_if::cfg_if;

pub fn setup_logging() {
    // enable logging and panic hook only during debug builds
    cfg_if! {
        if #[cfg(feature = "wasm-logger")] {
            wasm_logger::init(wasm_logger::Config::default());
        }
    }
}
