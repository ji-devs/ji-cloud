# Themes 

The theme settings are defined from ideation at [here](https://docs.google.com/spreadsheets/d/1kT35Q0AUuT5Fz8juPOW_-OVnIT08bF2E5RZK1vUI4Wc/edit?usp=sharing)

On the code side, this is applied across a few places:

1. CSS var definition: `elements/src/_themes/themes.ts`
2. Rust definitions and config loading: `crates/utils/src/themes.rs`
3. Rust config JSON: `config/themes.json`

Ideally the `themes.json` would be used in both the CSS var side and Rust, and would be the single source of truth for all settings, but that isn't working right now (just need to import directly into TS)

There are a few other places where compile-time definitions need to be made as new themes are added:

1. ThemeId in Rust: in the shared crate
2. ThemeIdExt and list of themes in Rust: `crates/utils/src/themes.rs`
3. ThemeKind in elements: `elements/src/_themes/themes.ts`
4. List of themes in Storybook: `components/module/_common/theme.ts`

Lastly, the various images are stored on the CDN in the ui/theme directory

Note that modules are free to use the above settings however they wish, e.g. to use a specific theme mapping based on runtime conditions