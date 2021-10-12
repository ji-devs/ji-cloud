cargo +nightly watch -w ../../../shared/rust -w ./src -w ./tests -x "test -- --nocapture"

cargo +nightly watch -w ../../../shared/rust -w ./src -w ./tests -x run
