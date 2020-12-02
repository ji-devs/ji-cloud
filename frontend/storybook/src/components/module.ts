import {renderTemplate as tmpl} from "@utils/template";
import {appendId, getChildId, } from "@utils/dom";
import moduleEditPlain from "@templates/module/_common/module-edit-page-plain.html";
import moduleEditResize from "@templates/module/_common/module-edit-page-resize.html";
import moduleEmpty from "@templates/module/_common/module-page-empty.html";
import modulePlayIframe from "@templates/module/_common/module-play-iframe.html";
import modulePlayIframePreview from "@templates/module/_common/module-play-iframe-preview.html";

export interface ModulePage {
    kind: ModulePageKind
    sidebar?: Element,
    header?: Element,
    main?: Element,
    footer?: Element,
}

export enum ModulePageKind {
    Empty,
    EditPlain,
    EditResize,
    PlayIframe,
    PlayIframePreview,
}

export interface ModulePageSections {
}

//sections are sidebar, header, main, and footer. They're all optional

export const modulePage = ({kind, sidebar, header, main, footer}:ModulePage) => {
    const html = kind == ModulePageKind.EditPlain ? moduleEditPlain
        : kind == ModulePageKind.EditResize ? moduleEditResize
        : kind == ModulePageKind.PlayIframe ? modulePlayIframe
        : kind == ModulePageKind.PlayIframePreview ? modulePlayIframePreview
        : kind == ModulePageKind.Empty ? moduleEmpty
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