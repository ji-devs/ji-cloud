import {renderTemplate as tmpl} from "@utils/template";
import {appendId, addClassesId, setAttributeId, setTextId} from "@utils/dom";
import {MEDIA_UI} from "@utils/path";
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
    const {page, sidebar} = basePage(); 
    appendId(page, "sidebar", sidebar);
    const moduleSelection = tmpl(ModuleSelection, {});

    const modules = Array(4).fill(0).map((_, idx) => makeModule(idx));

    appendId(page, "right-area", moduleSelection);

    return page;
}
export const Menu = () =>  {
    const {page, sidebar} = basePage(); 
    const menu = tmpl(menuSection, {});

    appendId(sidebar, "menu", menu);

    return page;
}

export const JigDelete = () =>  {
    const {page} = basePage(); 
    const deleteEl = tmpl(deletePopup, {});

    appendId(page, "delete-popup", deleteEl);

    return page;
}

function basePage() {
    const page = tmpl(editPage, { });
    const sidebar = tmpl(sidebarSection, {});
    appendId(page, "sidebar", sidebar);

    Array(4).fill(0).map((_, idx) => makeModule(idx))
        .forEach(module => {
            appendId(sidebar, "modules", module);
        });

    addClassesId(page, "hover-module", ["hidden"]);
    return {page, sidebar};
}

function makeModule(idx) {
    const module = tmpl(idx % 2 == 0 ? moduleLeft : moduleRight);

    setTextId(module, "title", `Title ${idx+1}`);
    setTextId(module, "subtitle", `Subtitle ${idx+1}`);

    if(idx == 1) {
        setAttributeId(module, "img", "src", `${MEDIA_UI}/Icn_Activity_Poster_124.svg`);
    }
    return module;
}
