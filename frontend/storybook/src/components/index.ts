import {loadAllFonts} from "@elements/_themes/themes";

//Not sure how to block the rest of storybook until this finishes...
//So there is probably a race condition with font loading.
//oh well!
//Interested elements can always re-load safely, and mock elements can be used in Storybook
//See theme-text / text-example 
loadAllFonts();