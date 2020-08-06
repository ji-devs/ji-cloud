import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import sidebar from "@templates/sidebar.html";

export default {
  title: 'Admin page',
}

export const Admin = storyAbout(
    "admin - temp", 
    () => tmpl(sidebar),
    `## Admin placeholder page`
);
