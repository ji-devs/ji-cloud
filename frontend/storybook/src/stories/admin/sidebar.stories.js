import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId} from "@utils/dom";
import sidebar from "@templates/admin/sidebar.html";

export default {
  title: 'Admin/Sidebar',
}

export const Sidebar = () => 
    tmpl(sidebar, {
      navbarLink: "Label images",
    });
