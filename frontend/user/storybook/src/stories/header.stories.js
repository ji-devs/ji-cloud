import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import header from "@templates/header.html";
import button from "@core/templates/button.html";

export default {
  title: 'Global Header',
}

export const User = storyAbout(
    "user", 
    () => tmpl(header, {
        signin: tmpl(button, {label: "profile"})
    }), 
    `## Logging in
      Happens with the auth system
    `
);

export const Guest = story(
    "guest", 
    () => tmpl(header, {
        signin: tmpl(button, {label: "sign in"})
    }), 
);
