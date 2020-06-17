# Frontend Developer's Pipeline

## STEP 1: HTML/CSS

1. Design gets build in Storybook with HTML and CSS (using Tailwind CSS)
2. Pushing this will bundle/minify the CSS and make it publically available
3. The Storybook app has a handcrafted plugin to generate Dominator code
4. Once components are properly approved - the Dominator code can be used

## STEP 2: RUST 

1. Copy/paste Dominator code from Storybook to Rust
2. Add event handling, logic, state management, etc.
3. Deploy (via CI/CD to run tests) 
