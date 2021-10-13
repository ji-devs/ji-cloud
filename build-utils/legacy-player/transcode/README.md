cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x "test -- --nocapture"

cargo +nightly watch -w ../../../frontend/apps/crates/utils -w ../../../shared/rust -w ./src -w ./tests -x run
