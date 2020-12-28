# Components 

Here we refer explicitly to the components created in Storybook for UI/UX prototyping.

----

### Import the element

It's a straight import of the code, not a module import (because it's executed right away and defines the custom element for usage by name).

Good:

```typescript
import "@elements/buttons/my-button";
```

Bad:

```typescript
import {MyButton} from "@elements/buttons/my-button";
```

### Provide arguments via controls

1. Always use default args and destructure _in_ the component (this allows re-using the component elsewhere)
2. Assign the default args to the components `args` property

Example:

```typescript
export default {
  title: 'Buttons',
}

interface ButtonArgs {
  text: string
}

const DEFAULT_ARGS:ButtonArgs = {
  text: "click me"
}

export const Button = (props?:ButtonArgs) => {
    const {text} = props || DEFAULT_ARGS;

    return `<my-button text="${text}" />`
}

Button.args = DEFAULT_ARGS;

```

If the element itself needs to be changed, but it uses the same basic arguments, re-use them:

```typescript
export const CircleButton = (props?:ButtonArgs) => {
    const {text} = props || DEFAULT_ARGS;
    return `<circle-button text="${text}" />`
}
export const RectButton = (props?:ButtonArgs) => {
    const {text} = props || DEFAULT_ARGS;
    return `<rect-button text="${text}" />`
}

CircleButton.args = DEFAULT_ARGS;
RectButton.args = DEFAULT_ARGS;

```

Of course that can also be made into a function, e.g.:

```typescript
const buttonArgs = (text) => ({...DEFAULT_ARGS, text});

CircleButton.args = buttonArgs("click a circle");
RectButton.args = buttonArgs("click a rectangle");
```

### Sometimes controls are abstract 

When the element needs a certain property to be set, but it makes more sense to provide a control in another format, feel free to do so.
This is usually going to be the case when a component is showing a larger composition as opposed to showing the element itself.

Example:

```typescript
import "@elements/pages/user-page";
import "@elements/buttons/my-button";

export default {
  title: 'Pages',
}

interface PageArgs {
  scenario: "login" | "register"
}

const DEFAULT_ARGS:PageArgs = {
  scenario: "login"
}

export const UserPage = (props?: PageArgs) => {
    const {scenario} = props || DEFAULT_ARGS;

    const color = scenario == "login" ? "red" : "blue";

    return `
        <user-page>
            <my-button color="${color}" />
        </user-page>
    `
}

UserPage.args = DEFAULT_ARGS;
```

### Define the control type

By default, Storybook will try to guess the control type, but it defaults to a string most of the time.

Set it explicitly for more control:

```typescript

//Continuing the previous example
UserPage.argTypes = {
  scenario: {
    control: {
      type: 'inline-radio',
      options: ["login", "registration", "profile"]
    }
  }
}
```

The current list of available controls and annotations are here: [https://storybook.js.org/docs/react/essentials/controls#annotation](https://storybook.js.org/docs/react/essentials/controls#annotation)