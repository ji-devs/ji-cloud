import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClassesId, setTextId} from "@utils/dom";
//these are the Add templates
import editPage from "@templates/jig/edit/edit-page.html";
import sidebarSection from "@templates/jig/edit/sidebar.html";
import menuSection from "@templates/jig/edit/menu.html";
import deletePopup from "@templates/jig/edit/delete-popup.html";
import moduleLeft from "@templates/jig/edit/sidebar-module-left.html";
import moduleRight from "@templates/jig/edit/sidebar-module-right.html";
import ModuleSelection from "@templates/jig/edit/module-selection.html";

export default {
  title: 'JIG/Edit',
}

export const EditPage = () => {
    const page = tmpl(editPage, { });
    const sidebar = tmpl(sidebarSection, {});
    const moduleSelection = tmpl(ModuleSelection, {});

    const modules = Array(4).fill(0).map((_, idx) => makeModule(idx));

    appendId(page, "sidebar", sidebar);
    appendId(page, "right-area", moduleSelection);

    modules.forEach(module => appendId(sidebar, "modules", module));

    return page;
}
export const Menu = () =>  {
    const page = tmpl(editPage, { });
    const sidebar = tmpl(sidebarSection, {});
    const menu = tmpl(menuSection, {});

    const modules = Array(4).fill(0).map((_, idx) => makeModule(idx));

    appendId(page, "sidebar", sidebar);
    modules.forEach(module => appendId(sidebar, "modules", module));
    appendId(sidebar, "menu", menu);

    return page;
}

export const JigDelete = () =>  {
    const page = tmpl(editPage, { });
    const sidebar = tmpl(sidebarSection, {});
    const deleteEl = tmpl(deletePopup, {});

    appendId(page, "sidebar", sidebar);
    appendId(page, "delete-popup", deleteEl);

    return page;
}

function makeModule(idx) {
    const module = tmpl(idx % 2 == 0 ? moduleLeft : moduleRight);

    setTextId(module, "title", `Title ${idx+1}`);
    setTextId(module, "subtitle", `Subtitle ${idx+1}`);
    toggleClassesId(module, "drag-border", ["hidden"], true);

    return module;
}