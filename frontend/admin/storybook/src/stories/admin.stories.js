import {renderTemplate as tmpl} from "@utils/render";
import sidebar from "@templates/sidebar.html";
import searchpage from "@templates/searchpage.html";
import categories from "@templates/_demo/categories.html";

export default {
  title: 'Admin Page',
}

export const Sidebar = () => 
    tmpl(sidebar, {
      navbarLink: "Label images",
    });


export const SearchPage = () =>
    tmpl(searchpage, {
      navbarLink: "Label images",
      results: 48,
    });

export const CategoryPage = () => tmpl(categories, {});

export const MainPage = () => tmpl(sidebar, { navbarLink: "Label images", });

