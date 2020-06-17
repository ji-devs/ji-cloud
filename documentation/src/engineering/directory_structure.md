# Directory structure

* _secret-keys: contains secret keys for local development (.gitignored)

* backend: servers and backend utils deployed to Cloud Run, Cloud Functions, etc.
  - api: the main api server (Rust/Warp/Diesel)
  - pages: the main html server (Rust/Handlebars)
  - api-js: supplementary api server for node sdks (JS)

* build-utils: internal tooling and utils

* documentation: this book

* frontend: projects that get compiled to Single Page Applications (Rust->Wasm)
  - _devfiles: common files needed for local development
  - user: things having to do with users (login, registration, etc.)

* shared: code that gets shared between different projects

* storybook: the storybook project to build html/css