import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import example from "@templates/temp/template-example.html";
import demo from "@templates/_demo/demo.html";

export default {
  title: 'Temp',
}

export const TemplateExample = story(
    "Template Example",
    () => tmpl(example, {
        navbarLink: "Label images",
        dynamicButtonLabel: "hello",
    }),

);


export const Demo = story(
    "Demo",
    () => tmpl(demo)
);
