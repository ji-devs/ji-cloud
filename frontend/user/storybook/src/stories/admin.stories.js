import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import sidebar from "@templates/admin/media-uploader/sidebar.html";
import button from "@core/templates/_buttons/button.html";

export default {
  title: 'Admin Page - image uploader',
}

export const MainPage = storyAbout(
    "Main Page",
    () => tmpl(sidebar, {
      navbarLink: "Label images",
  
      button: tmpl(button, {label: "Click"}),

  }),
    `## Main page of image uploader
    `
);
