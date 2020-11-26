import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import pageTmpl from "@templates/module/poster/edit/poster-edit.html";
import layoutSidebar from "@templates/module/poster/edit/sidebar/layout.html";
import layoutSidebarItem from "@templates/module/poster/edit/sidebar/layout-item.html";

export default {
  title: 'Modules/Poster/Edit',
}

export const Layout = () => {
    let page = corePage();
    let sidebar = tmpl(layoutSidebar);
  
    page = appendId(page, "sidebar", sidebar);

    return page;
}

function corePage() {
    let page = tmpl(pageTmpl, {
        title: "Create a Cover Page",
        subtitle: "Introduce your topic<br/>Use the blue panel for selecting layouts, themes, and adding content"
    }); 

    return page;
}