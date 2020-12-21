# Elements

### Consult the library guides for general rules

This document is a supplement, not a replacement, for the basic usage guides:

* [lit-element](https://lit-element.polymer-project.org/guide) 
* [lit-html](https://lit-html.polymer-project.org/guide)

----

### Don't name the classes

Rather, use a `_` as a placeholder:

```typescript
@customElement('circle-button')
export class _ extends LitElement { 
    //...
}
```

The exception to this is when defining a base class for the sake of inheritance (discussed below in _Global styles_)

### Be explicit with variants 

Example - a property "color" should not be defined as a string, rather it should be defined as an enum or string variants:

```typescript
@property()
color: "red" | "blue" | "green" = "red"; 
```

### No null values 

In the example above, it could have instead be written as:

```typescript
@property()
color: "red" | "blue" | "green" | null = null; 
```

This is bad. There should _always_ be a sane non-null value (for example, an empty string might be okay if that's a valid thing to render)


### Favor a declarative code style

Some tips:

* There is almost never a need for a `var` or `let`, everything should be `const`.
* Instead of switch+return, use conditionals (a.k.a. ternaries) or a predefined lookup
* Create pure functions instead of class methods (i.e. in `render()`, call `renderFooter(args)` instead of `this.renderFooter()`)

When conditionals get long, use the following style:

```typescript
cond1 ? option1
 : cond2 ? option2
 : cond3 ? option3
 : default
```

### Use nothing

When interpolating the template literals, it's often required to render "nothing". Instead of `null` or `""`, lit-html exports a special `nothing` object for this use case. Use that instead, since it will also prevent other bugs.

See [lit-html docs](https://lit-html.polymer-project.org/guide/writing-templates#rendering-nothing) for details.

### Don't hardcode data in the render function

For dynamic data, elements must of course accept it as a property and the data is set via the component.

However, even for static data, it should be moved out of the render function and defined as a `const`. For strings, use the `STR_` prefix in order to facilitate string replacement / localization later.

Example:

```typescript

const STR_HOWDY = "hello world";

@customElement('my-element')
export class _ extends LitElement {
  render() {
    return html`<div>${STR_HOWDY}</div>`;
  }
}
```

### Global styles - inheritance

Some things are global and used very often, like text weight options.

These should be created as base classes in `_styles` and then imported and extended as needed.

In fact it's probably a good idea to have a handful of base classes from which _all_ elements extend.

Example:

_in _styles/text.ts_

```typescript
export class BaseText extends LitElement {
  static get styles() {
    //Note that it's an array even with only one entry
    //This is required to keep the consistent ...super.styles in the subclass
    return [css`
        .bold{
            font-weight: 600;
        }
    `]
  }
}
```

_anywhere_
```typescript

import {BaseText} from "@elements/_styles/text";

@customElement('my-element')
export class _ extends BaseText {

    static get styles() {
        return [...super.styles, css`
            :host {
                width: 100px;
            }
        `]
    }
    render() {
        //...
    }
}
```

### Global styles - interpolation

Some styles are global in terms of their _value_ but not their _usage_. For example, we may have a particular shade of blue - but that blue is used in many contexts such as background, border, font, etc.

These should also exist in `_styles` but be defined as plain objects, where each value has the `css` tag on the literal, and then imported / used as-needed.

Example:

_in _styles/colors.ts_

```typescript

import { css} from 'lit-element';

export const colorValues = {
  grey: css`rgba(161,168,173,255)`,
  blue: css`rgba(85,144,252,255)`
} 
```

_anywhere_

```typescript

import {colorValues} from "@elements/_styles/colors";

@customElement('circle-button')
export class _ extends LitElement {

  static get styles() {
    return [css`
      .foo {
        border-style: solid;
        border-width: 1px;
        border-color: ${colorValues.grey}; 
      }
    `]
  }

  render() { 
      //... 
  }
}
```

### Compose elements together

It's absolutely fine to import other elements and compose them together in elements, not just components.

That said, if it leads to excessive "prop drilling" or "event retargeting", it's probably better to do that composition on the component level.