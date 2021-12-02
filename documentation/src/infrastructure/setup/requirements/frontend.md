# Frontend one-time setup

## Requirements

- NodeJS v15
    - [NodeJS](https://nodejs.org/en/)
    - Or with [nvm](https://github.com/nvm-sh/nvm/blob/master/README.md#installing-and-updating)
        - `nvm install v15 && nvm alias default v15`
- rustc nightly _after_ [edition 2021 was stabilized](https://github.com/rust-lang/rust/pull/88100#event-5229813170)
    - [Install](https://www.rust-lang.org/tools/install)
    - Update if necessary: `rustup default nightly && rustup update`
- [cargo-make](https://github.com/sagiegurari/cargo-make)
    - `cargo install cargo-make`
- [cargo-watch](https://github.com/watchexec/cargo-watch)
    - `cargo install cargo-watch`
- [watchexec-cli](https://github.com/watchexec/watchexec)
    - `cargo install watchexec-cli`

## Setup

1. Setup your Git credentials (probably easiest via [Github Desktop](https://desktop.github.com/))
2. [Fork the repo](https://github.com/ji-devs/ji-cloud)
3. Clone it into your local folder (using Github Desktop or manually)
4. Install frontend dependencies
    - `storybook` and `elements` projects require a Font Awesome auth token to
    install fonts:
        - `export FONTAWESOME_NPM_AUTH_TOKEN=<REPLACE_ME>`
    - `npm install` in the following folders:
        - `frontend/storybook`
        - `frontend/elements`
        - `frontend/apps`
        - `frontend/build-utils`
5. Install Dropbox / accept the invitation to ji-cloud-media  (we might move that over to a separate repo at some point...)

After all this is setup, you should be able to `npm start` from `frontend/storybook` and see it working, just without the images.

(the rest of the setup is merely setting the `.env` values)
