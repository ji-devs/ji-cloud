import {story, storyAbout} from "@utils/stories";
import {renderTemplate as tmpl} from "@core/js/render";
import sidebar from "@templates/sidebar.html";
import searchpage from "@templates/searchpage.html";
import categorypage from "@templates/categories/categories-page.html";

export default {
  title: 'Admin page',
}

export const Sidebar = story(
    "Sidebar",
    () => tmpl(sidebar, {
      navbarLink: "Label images",
    }),

);

export const SearchPage = story(
    "Search Page",
    () => tmpl(searchpage, {
      navbarLink: "Label images",
      results: 48,

    }),


);

export const CategoryPage = story(
    "Category Page",
    () => tmpl(categorypage, {

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
