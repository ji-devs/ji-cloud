import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClasses, toggleClassesId, setTextId} from "@utils/dom";
import editPage from "@templates/jig/shell/jig-module-edit-page.html";
import playPage from "@templates/jig/shell/jig-module-play-page.html";
import sidebarModuleSelected from "@templates/jig/shell/jig-sidebar-module-selected.html";
import sidebarModuleDeselected from "@templates/jig/shell/jig-sidebar-module-deselected.html";

export default {
  title: 'Jig/Shell',
}

export const Edit = () => {
    const page = tmpl(editPage , {

    });

    const modules = Array(4).fill(0).map((_, idx) => makeModule(idx));
    modules.forEach(module => appendId(page, "modules", module));

    setTextId(page, "module", "MODULE EDITOR HERE");

    return page;
}


export const Play = () => {
    const page = tmpl(playPage, {

    });

    const modules = Array(4).fill(0).map((_, idx) => makeModule(idx));
    modules.forEach(module => appendId(page, "modules", module));

    setTextId(page, "module", "MODULE PLAYER HERE");

    return page;
}


function makeModule(idx) {
    const module = tmpl(idx == 1 ? sidebarModuleSelected : sidebarModuleDeselected); 
    setTextId(module, "label", `${('0' + (idx+1)).slice(-2)}`);
    return module;
}
