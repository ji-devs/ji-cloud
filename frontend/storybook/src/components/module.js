import {renderTemplate as tmpl} from "@utils/template";
import {appendId, toggleClasses, getChildId, toggleClassesId, setTextId} from "@utils/dom";
import moduleEditPagePlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";
import modulePlayPage from "@templates/module/_common/module-play-page.html";

//sections are sidebar, header, main, and footer. They're all optional

export const ModuleEditPlainPage = (sections) => 
    appendSections(sections, tmpl(moduleEditPagePlain));

export const ModuleEditResizePage = (sections) => 
    appendSections(sections, tmpl(moduleEditPageResize));

export const ModulePlayPage = (sections) => 
    appendSections(sections, tmpl(modulePlayPage));

function appendSections(sections, page) {
    let {sidebar, header, main, footer} = sections;

    if(sidebar) {
        appendId(page, "sidebar", sidebar);
    }
    if(header) {
        appendId(page, "header", header);
    }
    if(main) {
        appendId(page, "main", main);
    }
    if(footer) {
        appendId(page, "footer", footer);
    }

    return {page, sidebar, header, main, footer};
}