import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import searchpage from "@templates/searchpage.html";

export default {
  title: 'Search Page',
}


export const SearchPage = () =>
    tmpl(searchpage, {
      navbarLink: "Label images",
      results: 48,
    });
