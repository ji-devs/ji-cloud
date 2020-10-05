import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import sidebar from "@templates/admin/sidebar.html";

export default {
  title: 'Admin/Main',
}

export const MainPage = () => tmpl(sidebar, { navbarLink: "Label images", });
