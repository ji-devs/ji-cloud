import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import example from "@templates/temp/template-example.html";

export default {
  title: 'Temp',
}

export const TemplateExample = story(
    "Template Example",
    () => tmpl(example, {
        navbarLink: "Label images",
        dynamicButtonLabel: "from story!",
    }),

);
