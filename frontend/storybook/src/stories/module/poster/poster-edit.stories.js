import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import sidebarTmpl from "@templates/module/poster/edit/sidebar/sidebar.html";
import headerTmpl from "@templates/module/poster/edit/header.html";
import footerTmpl from "@templates/module/poster/edit/footer.html";
import mainTmpl from "@templates/module/poster/edit/main.html";
import layoutSidebar from "@templates/module/poster/edit/sidebar/layout.html";
import layoutSidebarItem from "@templates/module/poster/edit/sidebar/layout-item.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";

export default {
  title: 'Modules/Poster/Edit',
}

export const Layout = () => {

    const {page, header, sidebar} = corePage();
    sidebar.append(tmpl(layoutSidebar));

    return page;
}

function corePage() {
    const page = tmpl(moduleEditPageResize);
    const sidebar = tmpl(sidebarTmpl);
    const header = tmpl(headerTmpl, {
        title: "Create a Cover Page",
        subtitle: "Introduce your topic<br/>Use the blue panel for selecting layouts, themes, and adding content"
    });
    const footer = tmpl(footerTmpl);
    const main = tmpl(mainTmpl);

    appendId(page, "sidebar", sidebar);
    appendId(page, "header", header);
    appendId(page, "footer", footer);
    appendId(page, "main", main);

    const wrapper = tmpl(`<div class="w-screen h-screen"></div>`);
    wrapper.append(page);
    return {page, sidebar, header, footer, wrapper};
}