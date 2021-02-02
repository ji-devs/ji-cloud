# Components 

Here we refer explicitly to the components created in Storybook for UI/UX prototyping.

----

### VSCode helpers

There are a couple snippets you can add to your VSCode config to automate the boilerplate for new components:

[VSCode snippets gist](https://gist.github.com/dakom/77e9b8299870b71512e55fb9222c4535)

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

### Don't hardcode data in the return function

Static data should be moved out of the return function and defined as a `const`. For strings, use the `STR_` prefix in order to facilitate string replacement / localization later (a similar technique is used in Elements).

Example (not including props for the sake of simplicity):

```typescript

const STR_HOWDY = "hello world";

export const MyStory = () => `<div>${STR_HOWDY}</div>`
```

Note that the next step which is currently unimplemented will be moving those strings into the config folder.
That will be an easy transition from the above, since it will _not_ require any change in the html,
and it should be easy to step through all the components (and elements) with the `STR_` prefix to switch over.

Future example:

```typescript
import {STR_HOWDY} from "~/config/strings";
```

### Controls

Generally speaking, use Controls (via the `args` property) to simulate data that changes at runtime. There is no need at all to re-simulate that dynamic data in other contexts.

For example, a standalone button story _should_ have a Control to see how that button behaves with all sorts of text.

Once that button is used in another component where the text is predefined, e.g. "Next", then it should _not_ be controllable and the above technique of static data applies.

### Provide arguments

1. Args should always be well-typed and optional (e.g. `foo(args?:Partial<MyArgs>)`)
2. A hardcoded default should be used as a fallback if no args are provided
3. To implement the fallback, destructure _in_ the component
4. Assign the default to the components `args` property (this makes it part of Storybook's Controls)
5. Enumerations should be expressed as actual enums or unions (not free-for-all strings/numbers) - and should similarly have a control type of radio, dropdown, etc.
6. Use `argsToAttrs()` to make life easier

Note that for the sake of jargon, "args" and "props" are used interchangeably, but we tend to use "args" on the outside since that fits with Storybook's lingo, and "props" on the inside since that fits with React/Component lingo.

Example:

```typescript
import "@elements/my-button";

export default {
  title: 'Buttons',
}

interface ButtonArgs {
  text: string
}

const DEFAULT_ARGS:ButtonArgs = {
  text: "click me"
}

export const Button = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `<my-button ${argsToAttrs(props)} />`
    //same as return `<my-button text="${props.text}" />`
}

Button.args = DEFAULT_ARGS;

```

Destructing into separate objects is straightforward:

```typescript
interface ButtonArgs {
  text: string,
  src: string,
}

const DEFAULT_ARGS:ButtonArgs = {
  text: "click me",
  src: "example.jpg",
}

export const Button = (props?:Partial<Args>) => {
    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    const {src, ...buttonProps} = props;

    return `
      <my-button ${argsToAttrs(buttonProps)}>
        <my-image src="${src}" />
      </my-button>
      `
}
```

### Sometimes controls are abstract 

One use case for stories/components is to show elements 1:1. Another is to show a larger composition, where controls need to be mapped.

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

Set it explicitly for more control. For example, this creates a radio selection:

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

### Slots

There is a pattern where you want a component to _optionally_ render its elements to a particular slot. 

(if it's not optional, just assign the slot attribute directly)

In order to make this optional assignment easier, there's a couple helper functions in `@utils/slot`.


`injectSlotStr` - will inject a `slotStr` property into the provided object with the html string of `slot="${slot}"`, if the object has a `slot` property. If it doesn't, then `slotStr` will be an empty string.

Example:

```typescript

const props = {
  name: "hello"
  slot: "foo"
}

const {name, slotStr} = injectSlotStr(props);
return `<div name="${name}" ${slotStr}></name>` // <div name="hello" slot="foo" />
```

That's helpful when there's exactly one property named `slot` in the props object, but when you have more than one, use `extractSlotStr`:


```typescript

const props = {
  name: "hello"
  slot1: "foo"
  slot2: "foo"
}

const {name} = props;
const slot1Str = extractSlotStr ("slot1") (props);
const slot2Str = extractSlotStr ("slot2") (props);

return `<div name="${name}" ${slot1Str}></name>` // <div name="hello" slot="foo" />
return `<div name="${name}" ${slot2Str}></name>` // <div name="hello" slot="bar" />
```

`extractSlotStr` is designed to make partial application easier:

```typescript

//imagine we use the name "left" for a lot of slots everywhere
//this could be added to the general utils module
const extractSlotLeft = extractSlotStr("left");

//And then used everywhere
const props = {
  name: "hello"
  left: "foo"
}

const {name} = props;
const slotStr = extractSlotLeft (props);

return `<div name="${name}" ${slotStr}></name>` // <div name="hello" slot="left" />
```