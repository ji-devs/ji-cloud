# Elements and Components

So now that we know why we're splitting these (and that we _can_ split them), how is it implemented?

## Elements

Elements are created in their own directory (`frontend/elements`). We use [lit-element](https://lit-element.polymer-project.org/guide) to make the elements easier to define, more typesafe, and fast. There are also lit-element specific tools and helper functions that can help in both coding and documenting.

These elements are imported directly into Storybook for building components, and are compiled into a javascript bundle for using elsewhere (such as the rust-powered live apps - but actually, not limited to that. We can use them in other venues too in order to keep consistent branding, for example)

Although the details of working in lit-element are outside the scope of this document, let's look at one quick point regarding CSS. The styles are encapsulated in the element completely. Yet we can also have global variabals to define things that genuinely need near-global reach such as theme colors and common font sizes. (see https://lit-element.polymer-project.org/guide/styles#expressions for example)

## Components (a.k.a. in Storybook - "Stories")

As mentioned before, for the purposes of quick iterative layout work, we use Storybook as a preliminary stage to create a guideline of how components are built out of the elements. At this level, its only purpose is to document and illustrate how the elements should be pieced together, as well as provide mechanisms for testing out how it might look with all sorts of custom data (as of this writing - we don't yet have GUI knobs and things, but we will)

The layout and technical design work is therefore split between building the elements in lit-element, and composing them for demo purposes in Storybook

Then, using this as a guideline, the final components that are coupled with real application state and event flow, are built in Rust/Dominator.

At this level of application building in Rust, the components may or may not be reusable exactly as-is. Those implementation details are outside the scope here, but on one foot, once the elements are easily reusable, it may not make so much sense for the components to be so generic due to the inherent complexity that introduces - it may be more sensible to rebuild the components for each specific context. Depends on the exact circumstances and tradeoffs. 