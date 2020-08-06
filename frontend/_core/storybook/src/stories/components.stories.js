import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import button from "@templates/button.html";

export default {
  title: 'Common Components',
}

export const Buttons = story(
    "Buttons", 
    () => `
        <div class="flex">
            <div class="m-3">${tmpl(button, {label: "button 1"})}</div>
            <div class="m-3">${tmpl(button, {label: "button 2"})}</div>
        </div>
    `
);
