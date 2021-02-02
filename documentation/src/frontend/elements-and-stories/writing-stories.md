# How to Write Stories 

More detail and code guidelines will be broken down [later](./code-style-stories.md), but for now let's talk about the overall concept.

We use Storybook for two purposes:

1. To preview and test elements. 
2. As a reference for app development.

That means the content in Storybook is temporary

## K.I.S.S.

Stories should generally be simple and stripped down to what's required for functionality only.

Some general tips:

* Do _not_ have static, real data in a story. It should always be in an element.
* _Do_ have dynamic, mock data in a story - to simulate what will happen at runtime.
* _Do_ use Controls and props to pass that mock data in.
* _Do_ keep the directory structure of stories mostly synced with elements
