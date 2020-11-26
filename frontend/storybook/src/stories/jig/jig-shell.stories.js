import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClasses, getChildId, toggleClassesId, setTextId} from "@utils/dom";
import editPage from "@templates/jig/shell/edit-page.html";
import playPage from "@templates/jig/shell/play-page.html";
import moduleEditPagePlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";
import modulePlayPage from "@templates/module/_common/module-play-page.html";
import editSidebarModule from "@templates/jig/shell/edit-sidebar-module.html";

export default {
  title: 'Jig/Shell',
}

export const EditPlain = () => {
    const jigPage = appendModules(tmpl(editPage));
    const modulePage = tmpl(moduleEditPagePlain);

    const sidebar = tmpl(`<div style="background-color: yellow;" class="h-full text-center">Module Sidebar</div>`);
    const header = tmpl(`<div style="background-color: red; color: white;" class="text-center">Header</div>`);
    const main = tmpl(`
    <div style="background-color: green;" class="w-full h-full flex flex-col justify-between">
        <div></div>
        <div style="color: white" class="w-full text-center">Main</div>
        <div></div>
    </div>
    `);
    const footer = tmpl(`<div style="background-color: blue; color: white;" class="text-center">Footer</div>`);

    appendId(modulePage, "sidebar", sidebar);
    appendId(modulePage, "header", header);
    appendId(modulePage, "main", main);
    appendId(modulePage, "footer", footer);

    appendId(jigPage, "module", modulePage);
    return jigPage;
}

export const EditResize = () => {
    const jigPage = appendModules(tmpl(editPage));
    const modulePage = tmpl(moduleEditPageResize);

    const sidebar = tmpl(`<div style="background-color: yellow;" class="h-full text-center">Module Sidebar</div>`);
    const header = tmpl(`<div style="background-color: red; color: white;" class="text-center">Header</div>`);
    const main = tmpl(`
        <div style="background-color: green;" class="w-full h-full flex flex-col justify-between">
        <div></div>
        <div style="color: white; font-size: 18rem" class="w-full text-center">Main</div>
        <div></div>
        </div>
    `);
    const footer = tmpl(`<div style="background-color: blue; color: white;" class="text-center">Footer</div>`);

    appendId(modulePage, "sidebar", sidebar);
    appendId(modulePage, "header", header);
    appendId(modulePage, "main", main);
    appendId(modulePage, "footer", footer);

    appendId(jigPage, "module", modulePage);
    return jigPage;
}
export const Play = () => {
    const jigPage = appendModules(tmpl(playPage));
    const modulePage = tmpl(modulePlayPage);
    const main = tmpl(`
        <div style="background-color: green;" class="h-full flex flex-col justify-between">
        <div></div>
        <div style="color: white; font-size: 18rem" class="w-full text-center">Main</div>
        <div></div>
        </div>
    `);
    appendId(modulePage, "main", main);
    appendId(jigPage, "module", modulePage);

    return jigPage;
}

function appendModules(page) {

    const sidebar = getChildId(page, "nav");

    Array(4).fill(0).map((_, idx) => {
        const module = tmpl(editSidebarModule);
        if(idx === 1) {
            toggleClasses(module, ["bg-jibackgroundGrey", "border-l-8", "border-jibuttonBlue"], true);
        } else {
            toggleClasses(module, ["border-jidarkgrey", "border-b"], true);
        }
        setTextId(module, "label", `${('0' + (idx+1)).slice(-2)}`);

        appendId(sidebar, "modules", module);
    });

    return page;
}
