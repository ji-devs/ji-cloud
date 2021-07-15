# Themes 

## Configuring existing themes

Themes are configured in `frontend/config/themes.json`

The various images are stored on the CDN in the ui/theme directory

## Adding new themes

Requires a couple extra steps:

1. ThemeId in Rust: in the shared crate
2. The as_str_id definition in the Rust themes.rs 

## Usage in code

The JSON is checked via Serde on the Rust side at `crates/utils/src/themes.rs`
It is also loosely checked on the element side via Typescript definitions at `elements/src/_themes/themes.ts`

These respectively do a bit more processing too (the element sets CSS vars and Rust sets up helpers to access data)

Note that modules are free to use the above settings however they wish, e.g. to use a specific theme mapping based on runtime conditions
