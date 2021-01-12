# Directory structure

* _secret-keys: contains secret keys for local development (.gitignored)

* backend: servers and backend utils deployed to Cloud Run, Cloud Functions, etc.
  - _core: common library used between backend apps
  - api: the main api server (Rust/actix-web/sqlx)
  - pages: the main html server (Rust/askama)
  - fastly-purge: cloud functions for purging the CDN on file change 
  - script: tools needed for backend stuff

* build-utils: internal tooling and utils for the project as a whole (e.g. connecting to db)

* documentation: this book

* frontend: projects that get compiled to Single Page Applications (Rust->Wasm)
  - apps/crates: the SPA Rust/dominator apps
    - entry: each entry point
    - utils: common utils for each app
    - components: reusable components between apps
  - build-utils: internal tooling and utils for frontend
  - config: configurating files for frontend
  - elements: lit-element custom elements ("web components")
  - ts-utils: typescript utils shared between frontend typescript
  - storybook: mock components for quick layout development and showcase

* shared: code that gets shared between frontend and backend 

* config: global configuration 
  - (js/rust): static settings
  - .env: local settings and secrets (not checked into repo)

