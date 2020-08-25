import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import sidebar from "@templates/sidebar.html";

export default {
  title: 'Main',
}

export const MainPage = () => tmpl(sidebar, { navbarLink: "Label images", });
