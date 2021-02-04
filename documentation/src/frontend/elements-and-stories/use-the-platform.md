# Use the platform

We only support modern, evergreen browsers, and in some specific cases don't even require supporting Safari or mobile.

Since custom elements are part of the web spec, this means the full power of the web api is available to us.

Here's a non-exuastive list of some possibilities

## Direct element references

Let's say you are rendering a canvas element like `<canvas id="canvas"></canvas>` and you need access to it in order to create a context.

You can call `this.shadowRoot.querySelector('#canvas')` in the `firstUpdated()` lifecycle method and it will return the `HTMLCanvasElement`.

The concept is identical to calling `document.querySelector()` on a webpage after the html has rendered, with the difference being that here it's scoped to the custom element where you call it. 

## CSS Grid

Since we are using encapsulated CSS, it means that we avoid frameworks that rely on global class definitions (bootstrap, tailwind, etc.)

In the next page we'll talk about [how we can tackle reusable styles in a number of ways](./code-style-elements.html#dynamic-styles), but for now - note that CSS grid is likely to be ubiquitous for creating container structures.

Named template areas with media queries, along with generic top-level structures with slots can effectively replace - and supercede - the grid capabilities of popular libraries, and it gives us far more control to create unique reusable patterns that fit our needs.

This article on MDN shows some examples: [https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout/Realizing_common_layouts_using_CSS_Grid_Layout](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout/Realizing_common_layouts_using_CSS_Grid_Layout)

## Mutation/Resize/Intersection Observers

These are powerful mechanisms that allow reacting to changes to all sorts of things, including DOM layout itself. They can be used to dynamically generate slot names based on child box size, set CSS vars to be passed down to children, and more.

