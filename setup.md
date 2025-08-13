

Clone repo

`cd ji-cloud`

`nvm install v20 && nvm alias default v20`

`rustup default nightly && rustup update`
 
`cd frontend/elements`

`cargo install cargo-watch`

`cd ../`

`cargo install --force cargo-make`

`cd ../build-utils`

`npm i`

`cd ../apps`

`npm i`

`rustup target add wasm32-unknown-unknown`

`rustup target add wasm32-unknown-unknown --toolchain nightly`

`rustup component add rust-std --target wasm32-unknown-unknown --toolchain nightly`

`rustup component add rust-src --toolchain nightly`

`rustup component add rustfmt --toolchain nightly`

`cargo make local-main home -Z macro-backtrace`

`cargo build`

`cargo make local-main home`



