import {renderTemplate as tmpl} from "@utils/template";
import {appendId, getChildId, } from "@utils/dom";
import moduleEditPagePlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditPageResize from "@templates/module/_common/module-edit-page-resize.html";
import modulePlayPage from "@templates/module/_common/module-play-page.html";

export interface ModulePage {
    kind: ModulePageKind
    sidebar?: Element,
    header?: Element,
    main?: Element,
    footer?: Element,
}

export enum ModulePageKind {
    EditResize,
    EditPlain,
    Play,
}

export interface ModulePageSections {
}

//sections are sidebar, header, main, and footer. They're all optional

export const modulePage = ({kind, sidebar, header, main, footer}:ModulePage) => {
    const html = kind == ModulePageKind.EditPlain ? moduleEditPagePlain
        : kind == ModulePageKind.EditResize ? moduleEditPageResize
        : kind == ModulePageKind.Play ? modulePlayPage
        : null; 

    if(!html) {
        throw new Error("unknown page!");
    }
  
    const page = tmpl(html);

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

    return page; 
}