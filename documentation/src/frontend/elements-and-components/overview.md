# Elements and Components

## Intro and reasoning

From a design perspective, _everything is a component_. Whether it's a single line of text with a specific font, or a full page consisting of hundreds of tiny pieces, the concept is all about composing pieces together in order to create larger structures - not as a hierarchy, but as an arbitrary mix of pieces, much like LEGO.

Once we move from a visual brief to the frontend engineering side, we have to consider another level of how these things compose together: dynamic properties and event propogation. 

By way of example, consider a single button with a label. Now let's imagine what happens if we have a menu containing a dozen of these buttons. If we want to change the text or color of precisely one of the menu's sub-buttons, how do we know which one to target? Furthermore, if a button is clicked, how do we know _which_ one was clicked?

In the old days, we would do this _imperatively_, by getting access to an element that we manually added to the DOM (e.g. via jQuery). This isn't necessarily a bad thing, but it becomes hard to maintain at scale and the more modern trend in web development is to favor _declarative_ frameworks like React, Vue, etc.

However, these frameworks introduce a level of indirection which require using special domain-specific language (JSX, v-* properties, Svelte, etc.), and then this forces the components to be tied to that specific framework.

That first gulp of Kool-Aid may not seem so bad, because they go to great lengths to make it feel similar to HTML. In fact it's often hard to tell the difference at first glance, and semantically, they often are an implementation of valid html (well... debatably... sure classNames is valid syntax... but it doesn't do anything outside of JSX/React)

In our case, it's a dealbreaker, because our framework of choice makes no effort at all to pretend to be HTML. In fact it's not even Javascript (we're using a Rust/WASM approach which has no JSX-like solution).

Yet... it turns out this is a good thing. Because _components_ are framework-specific, but _elements_ are not.

## What's the difference?

Let's revisit the second paragraph above:

> Once we move from a visual brief to the frontend engineering side, we have to consider another level of how these things compose together: dynamic properties and event propogation. 

Well it turns out that this is not always the problem it sounds like once we have `Custom Elements` (this is literally a web standard... most implementations tend to focus on combining it with other features in order to form `Web Components`, but for the sake of clarity, I will refer to `Custom Elements` even with shadow dom and slots as `Elements`, _not_ `Components`)

### Dynamic properties.

First of all - if there's no nesting, it's not a problem at all. You can simply set a property on the element and it will render accordingly.

Now, let's consider a case of nesting. Coming back to our menu with sub buttons, we want to change the text of one specific sub button.

Our answer here is to realize that because we need to do that, the menu is no longer just an element - it's now a composition of one element (the menu skeleton container) with other elements (individual buttons).

This may take some time to sink in - because it's not just the composition of a random container like a div... the menu _should_ be a real element, albeit one with no content defined.

By using `slots` we can give the menu some content. Specifically, at the _component_ level we give it some content.

In other words, if we have a `<my-menu>` element and `<my-button>` elements, then the component will look something like:

```
<my-menu>
  <my-button slot="buttons">Button 1</my-button>
  <my-button slot="buttons">Button 2</my-button>
  <my-button slot="buttons">Button 3</my-button>
</my-menu>
```

(it's a bit different in our framework's syntax, but it's fundamentally the same thing)

This works because the `my-menu` _element_ will have declared the `buttons` slot in its markup. That's why it has to be a real element, not just a random container (of course if has no stylying at all and the only purpose is to contain children, it can be any other element - div, ul, etc.)

### Event propogation

It's actually the same pattern, but with a bit more nuance due to event bubbling.

Starting off with the simple case again - no nesting, no problem, just target the element itself and all is good.

However, due to the way event bubbling works, we also have the case of where it's _effectively_ flattened and we don't care. For example, if I have a modal containing an image, a caption, and a button, and the _only_ thing I care about in terms of a click event is the button, then it doesn't matter if that click event is detected on the modal itself (as long as it's only _fired_ via the button - and, as an aside, it might be a good idea to give it a different event name like "close"). So in this case too, there's no need for a separate composition on the component level - an element is enough.

So when do we need a component-level abstraction for events? Same concept as before - when we have to _get at_ nested children. For example, detecting _which_ button among a number of nested children is the one which fired an event.

In our actual declarative framework it's different than actually adding onClick handlers in html, but we can think of it as nothing more than the same concept as above - we slot our children on the component level and add listeners to them:


```
<my-menu>
  <my-button slot="buttons" onclick=${handleClick(1)} >Button 1</my-button>
  <my-button slot="buttons" onclick=${handleClick(2)} >Button 2</my-button>
  <my-button slot="buttons" onclick=${handleClick(3)} >Button 3</my-button>
</my-menu>
```