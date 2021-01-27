# Checklist

The list here is meant to be consulted before a PR is submitted/merged to `sandbox`:

### Elements

  - Headspace: "raw materials to be used in building the app"

  - All the static data for the element should be hardcoded - not properties

  - Compose elements within elements - try to keep things DRY

  - Element name matches file name

  - No unused properties

  - All properties have the correct attribute conversion type

  - Use enums or unions instead of primitive types where appropriate

  - Sane defaults (don't rely on the Storybook mock to set a property)

  - Displayable strings should be outside of the render function (or at least outside the return `html`) and prefixed with `STR_`

  - Use lit-html's `nothing` instead of `null` or `""` when inside an html template

  - Use the suggested conditional syntax when there are multiple conditions

  - Use top-level CSS Vars for colors

  - Use inheritance where appropriate (such as re-using styles)

Tip: 

  - Plan the architecture carefully and consider how the element is intended to be used, not just as a wrapper for a bunch of HTML/CSS. Container elements with nothing other than slots and styles are absolutely fine, as are static elements that have no properties. Complex large elements that abstract over a ton of functionality are fine too if that's what's needed. For example, when separating this way, you may find that "my-custom-page" gets broken into a generic re-usable container which has nothing to do with that specific page, and only needs to be slotted with the custom content.

### Components / Stories

  - Headspace: "temp mockups/prototypes for Dominator reference" and/or "Element Tests"

  - Should mostly be about configuring and composing elements for interactivity. Move static data to the appropriate element.

  - If it corresponds to an element, names and directory structure should match

  - Storybook title nesting should match directory structure

  - Args/Props should be defined properly (tip: use the snippet)

  - Set the appropriate Control type (e.g. radios for unions/enums)

  - For now - prefer using HTML directly instead of importing and re-using component functions (makes it easier to copy/paste the HTML for now, due to a bug in the "view source" functionality in Storybook)

Tip:

  - Think of Storybook as like a visual approach to TDD. It's only a way to test and see the elements, and how they compose together - not where you _build_ the elements.


### Both

  - File is in the correct directory structure

  - STR_* is only for displayable strings and will eventually be moved to an external tool for copywriter/translater administration. For other strings like image paths, just use literals (and this should almost always be in the elements, not components)

  - Cleanup temp code. It's okay to commit it, but clean it up with another commit before making a PR
