import {story, storyAbout} from "@utils/stories";
import {renderTemplate} from "@common/templates";
import header from "@templates/header.html";

export default {
  title: 'Global Header',
}

export const User = storyAbout(
    "user", 
    () => renderTemplate(header, {signin: ""}), 
    `## Logging in
      Happens with the auth system
    `
);

export const Guest = story(
    "guest", 
    () => renderTemplate(header, {signin: "sign in here"})
);
