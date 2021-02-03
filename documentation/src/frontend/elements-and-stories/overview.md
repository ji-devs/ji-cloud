# Elements and Stories 

## Intro and reasoning

We tend to say everything is a _component_. Whether it's a single line of text with a specific font, or a full page consisting of hundreds of tiny configurable pieces, the concept is all about composing pieces together in order to create larger structures, much like LEGO.

However, once we move from a visual brief to the engineering side, we have to consider the implementation details of exactly how these things compose together and the limitations of our target platform. How do properties change? How do events propogate? How do children get swapped out? 

By way of example, consider a single button with a label. Now let's imagine what happens if we have a menu containing a dozen of these buttons. If we want to change the text or color of precisely one of the menu's sub-buttons, how do we know which one to target? Furthermore, if a button is clicked, how do we know which one was clicked?

In the old days, we would do this _imperatively_, by getting access to an element that we manually added to the DOM (e.g. via jQuery). This isn't necessarily a bad thing, but it becomes hard to maintain at scale and the more modern trend in web development is to favor _declarative_ frameworks like React, Vue, etc.

This declarative approach requires that we compose our components from the top down, e.g. a page contains sections, sections contain inputs, and so on.

## Web components

Historically speaking, the web did not have a mechanism for defining components, and so these frameworks introduce their own special domain-specific language (JSX, Vue properties, Svelte, etc.). In some cases, like JSX, this language is very close to HTML, and in others it's radically different - but in all cases, the language is tied to the framework. You can't take JSX and stick it on a webpage - you need React to render it.

This changed with the advance of "Web Components". The full spec and history of that is outside the scope here, but ultimately, it allows defining custom html elements that are literally html elements. You can stick them on a webpage and it just works, with no need for another framework.

## Lit-Element

That said, authoring web components directly is annoying and prone to performance problems. For that reason, it makes sense to author the web components in a framework that removes boilerplate and performs well at scale. Our choice is [lit-element](https://lit-element.polymer-project.org/) and its sister [lit-html](https://lit-html.polymer-project.org/).

## "Elements" not "Components"

Since _everything_ is a component, and it's also a term used in other contexts (like in the design brief), for now on we will refer to "web components" as "custom elements" or just "elements" for short.

This is technically accurate enough - "custom elements" are part of the "web component" spec, and we are ultimately rendering these custom elements.

## Storybook

Creating the elements is one thing, but we still need to see how they behave for QA purposes, and it's much faster to iterate on that before it gets bogged down with all the other requirements of the live app. In order to test with mock data, and prototype before signing off on the elements for further application development, we use Storybook as a visual testing environment.

The purpose of storybook is _only_ for QA/testing. Content created in Storybook alone is never bundled into the final app.

## "Stories" not "Components"

Again, it's not wrong to think of every story as a "component", and it even uses React under the hood - but to disambiguate, we will stick to calling them "stories".
