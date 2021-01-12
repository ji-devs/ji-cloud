# Frontend Templates (TODO - DEPRECATE)

The template system is based on a two-step process:

1. Robust pre-processing at build time
2. Simple key replacement at runtime

## Build Time

The pre-processing step uses a jinja2 like system, powered by [Tera](https://tera.netlify.app/docs)

Most of the capability should work out of the box.

At build time, the templates are collated from 3 locations and then output to one place (`.template_output/` which is gitignored). This output folder is made available to storybook via the `@templates` import alias. Original directory structure from each location is preserved.

The location of source templates are:
  * `frontend/core/templates` - common reusable templates for use in multiple projects
  * `frontend/[APP]/templates` - the templates for the specific app
  * `frontend/[APP]/storybook/demo-templates` - templates that are only used for demo purposes in storybook (these won't be available to the APP itself at build time)

Due to [an issue in Tera](https://github.com/Keats/tera/issues/547), the guideline is that `_core` and `demo-templates` should be nested in subdirectories prefixed with an underscore (to avoid namespace conflicts with eachother and the app templates). This is probably a good idea anyway for `_core` to keep things organized (e.g. in subdirectories like `_buttons`, `_input`, etc.). Since the Tera issue might one day be resolved, it's recommended to create a `_demo` subdirectory in `demo-templates`, and then reference it in the storybook code via `@templates/_demo/*`. That way, we can simply move the files without changing any of the storybook code if Tera adds the feature request.

In terms of Context, the only key passed down is `MEDIA_UI` (and this is changed via the build system to target local, sandbox, or release)

One thing which does not work is mixing macro definitions in the same file as a template. Rather, macros should be in their own file - and these will be filtered out when rendering.

## Runtime

At runtime, the only thing which is supported is simple key/value replacement. This follows a different pattern than the one supported by Jinja/Tera: `${pattern}`

The different runtimes do the actual replacement via:

  * JS: `renderTemplate()` in `_core/js/utils` (usually aliased as `tmpl`)
  * Rust: [simple-html-template](https://crates.io/crates/simple-html-template)

The JS function is potentially insecure, but it's only used in Storybook, (i.e. dev mode only where we have strict control over the values, so it's fine)

When using in the deployed app, the `simple-html-template` crate provides a `html_map!` and `html_map_strong!` macro which not only make it more convenient to create the hashmap, but also escape according to standard security rules (extra protection when setting attributes)

## Combo

The above system can be combined to allow passing dynamic values down to a template macro, allowing for a combination of static and dynamic replacements. 

For example:

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
