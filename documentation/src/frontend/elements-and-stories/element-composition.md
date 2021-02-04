# How to Compose Elements

## The Zen of Composition

Developing an intuition for how to structure elements is part art and part science. A general tip is to think through the big picture, consider how the element is used, and what its purpose is. Careful thought and planning can prevent  much refactoring later.

## Slots

Slots are how we compose elements together.

Let's pretend we have two elements already defined: `my-menu` and `my-button`

What we want is to have this simple html:

```
<my-menu>
  <my-button>Button 1</my-button>
  <my-button>Button 2</my-button>
  <my-button>Button 3</my-button>
</my-menu>
```

This works perfectly, with no changes to the above, if `my-menu` exposes an unnamed `slot`. For example, `my-menu` might render:

```
<div>
    <slot></slot>
</div>
```

But let's say `my-menu` wants to place those buttons in some specific nested area. That's easy enough, use a named slot:


```
<div>
    <h1>Menu Start<h1>
    <slot name="items"></slot>
    <h1>Menu End<h1>
</div>
```

And then, our top-level html is:


```
<my-menu>
  <my-button slot="items">Button 1</my-button>
  <my-button slot="items">Button 2</my-button>
  <my-button slot="items">Button 3</my-button>
</my-menu>
```

## Avoid uber-elements, embrace purposeful elements

Lit-element is provides a lot of expressive power by using properties to render dynamic content.
In fact, if one were so inclined, they could build an entire website in one ridiculous giant element.

Of course, this is a bad idea. The goal should be make elements _purposeful_ as well as to provide a simple API so that code is clean and elements can be re-used. Purpose here not necessarily mean "achieve the design brief", though it can be just that. It can mean "create a grid structure with slots", "control all the colors and fonts of children", etc. 

A page element specifying nothing other than a grid serves a clear purpose. So does a multimedia element or a controlbar. An uber-page element that does all of that together might not serve a clear purpose, would have a confusing API, and would be much better separated into pieces.

So how do we know when to split elements apart?

## Split at nested functionality

A good rule of thumb is to think about whether or not you need to _get at_ the children from the outside. For example, to set properties or respond to events.

Let's consider our above example of `my-menu` and `my-button`. Why did we split it, instead of just slogging those buttons in between `Menu Start` and `Menu End`? 

In a real-world example, it's likely that `Button 1` will have its text determined at runtime. So the alternative would mean `my-menu` would need to provide a mapping of a property for each button text (`button1Text`, `button2Text`, etc.). Ewww. Awful.

Also, in a real-world example, we'd want to do something when the button is clicked. So from the outside, we need to know which button is clicked. It's much cleaner to attach a listener to each button directly than to inspect the event target or have some hodgepodge of custom events. 

By splitting the button into its own element, and then slotting it in, we have a much cleaner API overall.

## Otherwise, don't split

If there is no need to _get at_ the children, then from the outside it's just one big opaque element and should be created as such. Breaking things into a million unnecessary pieces creates confusion and complexity.

Consider an element with some visual decorators like lines and boxes. It should all just be self-contained in that element - splitting it out and then requiring it to be slotted back in is utterly pointless.

In general, when there's a very clear mapping of properties to the element contents, or the element dispatches a very clear set of events, there is no need to split.

