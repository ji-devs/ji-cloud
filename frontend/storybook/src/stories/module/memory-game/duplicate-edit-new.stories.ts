import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockWords, mockThemes} from "./common/mock-data";
import {appendCardPairsTextText} from "./common/card-pairs";
import {modulePage, ModulePageKind} from "@components/module";
import sidebarTmpl from "@templates/module/memory/edit/duplicate/duplicate-edit-new.html";
import headerTmpl from "@templates/module/memory/edit/temp/temp-header.html";
import mainTmpl from "@templates/module/memory/edit/temp/temp-main.html";
import footerTmpl from "@templates/module/memory/edit/temp/temp-footer.html";

export default {
  title: 'Modules/Memory-Game/Edit/DuplicateNew',
}

export const Duplicate_Step_1 = () => {
    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar: makeSidebar(),
        header: makeHeader(),
        main: makeMain({
            pairKind: "text-text",
            flipSecond: false,
            isEdit: true,
            themeIndex: 1
        }),
        footer: makeFooter(),
    })
}

function makeSidebar():Element {
    const el = tmpl(sidebarTmpl);



    return el;
}

function makeHeader():Element {
    const el = tmpl(headerTmpl);

    return el;
}

interface MainOptions {
    pairKind: "text-text" | "text-image",
    flipSecond: boolean,
    isEdit: boolean,
    themeIndex: number
}

function makeMain({pairKind, flipSecond, isEdit, themeIndex}:MainOptions):Element {
    const el = tmpl(mainTmpl);

    if(pairKind == "text-text") {
        appendCardPairsTextText(el, {flipSecond, isEdit, themeIndex });
    }
    return el;
}

function makeFooter():Element {
    const el = tmpl(footerTmpl);
    return el;
}


function mockStep1(_page) {
    const page = appendCardPairsTextText(_page, {
        flipSecond: false,
        isEdit: true,
        themeIndex: 0
    });
    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });

    return page;
}
