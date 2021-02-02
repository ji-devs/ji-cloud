# Code Style - Elements

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
export type Color = "red" | "blue" | "green";

@property()
color: Color = "red"; 
```

### No null values 

In the example above, it could have instead be written as:

```typescript
@property()
color: "red" | "blue" | "green" | null = null; 
```

This is bad. There should _always_ be a sane non-null value.


### Favor a declarative code style

Some tips:

* There is almost never a need for a `var` or `let`, everything should be `const`.
* Instead of switch+return, use conditionals (a.k.a. ternaries) or a predefined lookup
* Create pure functions instead of class methods (i.e. in `render()`, call `renderFooter(args)` instead of `this.renderFooter()`)
* Split things out into small functions as needed

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

Static data should be moved out of the render function and defined as a `const` with an all-caps variable name. 

For displayable strings, we will eventually have a more complex localization solution, but for now - use the `STR_` prefix in order to facilitate string replacement / localization later

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

## Dynamic styles

lit-element and lit-html provide some helpers like `classMap` and `styleMap` to make some of the code around dynamic styling cleaner.

At the end of the day, you're just returning html. Both classes and inline styles can be changed at runtime based on properties.

## Reusable styles 

There are a few approaches to reusable static styles: css vars, inheritance, interpolation, and mixins.

### Reusable styles - CSS Vars

CSS Vars pierce down through the shadow dom, and can be used by any nested child anywhere.

It's a great way to define global themes, or, more generally, for a child element to declare which of its styles can be overridden by an ancestor

### Reusable styles - Inheritance

This approach is handy when there's a clear hierarchy of global styles for particular kinds of elements, but not others.

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

### Reusable styles - Mixins

Mixins are more flexible than inheritance, but can also be harder to reason about and decide what should/should not be included.

Example:

_in _styles/colors.ts_

```typescript

import { css} from 'lit-element';

export const colorTheme = css`
    .red { rgba(161,168,173,255) }
    .blue { rgba(85,144,252,255) }
`;
  
```

_anywhere_

```typescript

import {colorTheme} from "@elements/_styles/colors";

@customElement('circle-button')
export class _ extends LitElement {

  static get styles() {
    return [colorTheme, css`
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

Of course, they are just JS objects, so they can be grouped into their own arrays and mixed in like:

```
//cssThemes is an array of css objects
static get styles() {
    return [...cssThemes, css`
      .foo {
        border-style: solid;
        border-width: 1px;
        border-color: ${colorValues.grey}; 
      }
    `]
}
```

### Reusable styles - Interpolation 

The static CSS getter cannot be interpolated at runtime with dynamic values, but it can be interpolated with static `css` literals 

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
