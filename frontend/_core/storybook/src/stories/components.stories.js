import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import components from "@templates/_demo/components.html";

export default {
  title: 'Common Components',
}

export const Components = story(
    "Buttons", 
    () => tmpl(components)
);
