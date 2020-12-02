import {renderTemplate as tmpl} from "@utils/template";
import {appendId, getChildId, addClasses, addClassesId} from "@utils/dom";
import containerTmpl from "@templates/admin/container.html";
import sidebarLinkTmpl from "@templates/admin/sidebar-link.html";
import sidebarLinkLockedTmpl from "@templates/admin/sidebar-link-locked.html";

const LINKS = [
    {label: "Images", sidebarId: "images", locked: false},
    {label: "JIGs", sidebarId: "jigs", locked: false},
    {label: "Categories", sidebarId: "categories", locked: true},
];

export const withContainer = ({page, sidebarId}) => {
    const container = tmpl(containerTmpl);
    const sidebar = getChildId(container, "sidebar");

    LINKS.forEach(({label, locked, ...link}) => {
        const linkElem = locked 
            ? tmpl(sidebarLinkLockedTmpl, {label})
            : tmpl(sidebarLinkTmpl, {label, href: "#"});

        if(link.sidebarId == sidebarId) {
            addClasses(linkElem, "text-white");
        }
        appendId(sidebar, "links", linkElem);

    });
    appendId(container, "main", page);
    return container;
}
