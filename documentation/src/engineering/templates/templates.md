# Frontend Templates

The template system is based on a two-step process:

1. Robust pre-processing at build time
2. Simple key replacement at runtime

## Build Time

The pre-processing step uses a jinja2 like system, powered by [Tera](https://tera.netlify.app/docs)

Most of the capability should work out of the box.

Note that each SPA will get its templates, and also the shared core templates in `frontend/_core/templates/`. These core templates should be in subdirectories prefixed with an underscore (to avoid namespace conflicts)

In terms of Context, the only key passed down is `MEDIA_UI`

One thing which does not work is mixing macro definitions in the same file as a template. Rather, macros should be in their own file - and these will be filtered out when rendering.

## Runtime

At runtime, a different pattern than the one supported by Jinja/Tera is used: `${pattern}`

This is done via separate runtime utilities:

  * JS: `renderTemplate()` in `_core/js/utils`
  * Rust: [simple-html-template](https://crates.io/crates/simple-html-template)

The JS function is potentially insecure, but it's only used in Storybook, (i.e. dev mode only where we have strict control over the values, so it's fine)

When using in the deployed app, the `simple-html-template` crate provides a `html_map!` and `html_map_strong!` macro which not only make it more convenient to create the hashmap, but also escape according to standard security rules (extra protection when setting attributes)

## Combo

The above system can be combined to allow passing dynamic values down to a template macro. For example:

Given the following macro in `_core/templates/_buttons/button-macros.html`:

```
{% macro orange(label, dataId="") %}
<button data-id="{{dataId}}">
    {{label}}
</button>
{% endmacro orange %}
```

We can do the following, to either hardcode a value via the template system or allow a dynamic value to pass through:
```
{% import "_buttons/button-macros.html" as ButtonMacros %}
{{ ButtonMacros::orange(label="HardCoded!", dataId="btn-1") }}
{{ ButtonMacros::orange(label="${dynamicButtonLabel}", dataId="btn-2") }}
```