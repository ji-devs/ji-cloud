import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import sidebar from "@templates/sidebar.html";

export default {
  title: 'Admin page',
}

export const Sidebar = story(
    "Sidebar",
    () => tmpl(sidebar, {
      navbarLink: "Label images",
}),

);

export const MainPage = storyAbout(
    "Main Page",
    () => tmpl(sidebar, {
      navbarLink: "Label images",


  }),
    `## Main page of image uploader
    `
);
