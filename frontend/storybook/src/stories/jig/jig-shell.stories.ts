import {renderTemplate as tmpl} from "@utils/template";
import {appendId, addClasses, getChildId,setTextId, setIframeContentsId} from "@utils/dom";
import {modulePage, ModulePageKind} from "@components/module";
import editPage from "@templates/jig/shell/edit-page.html";
import playPage from "@templates/jig/shell/play-page.html";
import editSidebarModule from "@templates/jig/shell/edit-sidebar-module.html";


export default {
  title: 'Jig/Shell',
}

export const EditShell = () => {
    const page = appendModules(tmpl(editPage));


    setIframeContentsId(page, "iframe", "<html><body><h1>Module Editor Here!</h1></body></html>");

    return page;
}

export const PlayShell = () => {
    const page = appendModules(tmpl(playPage));

    setIframeContentsId(page, "iframe", "<html><body><h1>Module Player Here!</h1></body></html>");

    return page;
}

export const ModuleEditPlain = () => {
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

    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar,
        header,
        main,
        footer,
    })
}

export const ModuleEditResize = () => {

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

    return modulePage({
        kind: ModulePageKind.EditResize,
        sidebar,
        header,
        main,
        footer,
    })
}
export const ModulePlay = () => {
    const main = tmpl(`
        <div style="background-color: green;" class="h-full flex flex-col justify-between">
        <div></div>
        <div style="color: white; font-size: 18rem" class="w-full text-center">Main</div>
        <div></div>
        </div>
    `);

    return modulePage({
        kind: ModulePageKind.Play,
        main,
    })
}


function appendModules(page) {

    const sidebar = getChildId(page, "nav");

    Array(4).fill(0).map((_, idx) => {
        const module = tmpl(editSidebarModule);
        if(idx === 1) {
            addClasses(module, ["bg-jibackgroundGrey", "border-l-8", "border-jibuttonBlue"]);
        } else {
            addClasses(module, ["border-jidarkgrey", "border-b"]);
        }
        setTextId(module, "label", `${('0' + (idx+1)).slice(-2)}`);

        appendId(sidebar, "modules", module);
    });

    return page;
}
