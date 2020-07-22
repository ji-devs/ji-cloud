import {story, storyAbout} from "@utils/stories";
import {renderTemplate} from "@core/js/render";
import img_element from "@templates/tests/image_element.html";
import img_css from "@templates/tests/image_css.html";

export default {
  title: 'Tests',
}

export const ImageElement = storyAbout(
    "image element", 
    () => renderTemplate(img_element),
    "An image as HTML element"
);

export const ImageCss = storyAbout(
    "image css", 
    () => renderTemplate(img_css),
    "An image as CSS background-image"
);
