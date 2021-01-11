# Frontend one-time setup

1. Setup your Git credentials (probably easiest via [Github Desktop](https://desktop.github.com/))
2. [Install Rust](https://www.rust-lang.org/tools/install)
3. [Install Node](https://nodejs.org/en/)
4. Install cargo-make: `cargo install --force cargo-make`
5. Install cargo-watch: `cargo install --force cargo-watch`
6. Install watchexec: `cargo install --force watchexec`
7. [Fork the repo](https://github.com/ji-devs/ji-cloud)
8. Clone it into your local folder (using Github Desktop or manually) 
9. `npm install` in the following folders:
    * `frontend/storybook`
    * `frontend/elements`
    * `frontend/apps`
    * `frontend/build-utils`
    * `config/typescript`
10. Install Dropbox / accept the invitation to ji-cloud-media  (we might move that over to a separate repo at some point...)
11. Build the TS config dependencies
    * inside `config/typescript`: `npm run build`

After all this is setup, you should be able to `npm start` from `frontend/storybook` and see it working, just without the images.

(the rest of the setup is merely setting the `.env` values)