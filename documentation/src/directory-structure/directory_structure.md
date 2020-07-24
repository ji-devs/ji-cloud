# Directory structure

* _secret-keys: contains secret keys for local development (.gitignored)

* backend: servers and backend utils deployed to Cloud Run, Cloud Functions, etc.
  - _core
    - (js/rust): common library used between backend apps
  - api: the main api server (Rust/Warp/Diesel)
  - pages: the main html server (Rust/Handlebars)
  - api-js: supplementary api server for node sdks (JS)
  - fastly-purge: cloud functions for purging the CDN on file change 

* build-utils: internal tooling and utils

* documentation: this book

* frontend: projects that get compiled to Single Page Applications (Rust->Wasm)
  - _core
    - (js/rust): common library used between frontend apps
    - devfiles: files needed for local development
    - templates: common templates used between frontend apps 
  - [spa]: each Single Page Application
    - app: frontend wasm code
    - storybook: storybook reference project
    - templates: html templates

* shared: code that gets shared between frontend and backend 

* config: global configuration 
  - (js/rust): static settings
  - .env: local settings and secrets (not checked into repo)

