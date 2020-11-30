import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockWords, mockThemes} from "./common/mock-data";
import {appendCardPairsTextText} from "./common/card-pairs";
import {modulePage, ModulePageKind} from "@components/module";
import sidebarEmpty from "@templates/module/memory/edit/edit-new/duplicate/step1-sidebar-empty.html";
import sidebarWords from "@templates/module/memory/edit/edit-new/duplicate/step1-sidebar-words.html";

import headerPlain from "@templates/module/memory/edit/edit-new/header-plain.html";

import mainTmpl from "@templates/module/memory/edit/temp/temp-main.html";
import mainEmpty from "@templates/module/memory/edit/edit-new/main-empty.html";
import mainWords from "@templates/module/memory/edit/edit-new/duplicate/main-cards-words.html";

import footerPlain from "@templates/module/memory/edit/edit-new/footer-plain.html";

export default {
  title: 'Modules/Memory-Game/Edit/DuplicateNew',
}

export const Duplicate_Step_1_Empty = () => {
  const main = tmpl(mainWords);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);
    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar: tmpl(sidebarEmpty),
        header,
        main: makeMain({
            pairKind: "text-text",
            flipSecond: false,
            isEdit: true,
            themeIndex: 1
        }),
        footer,
    })
}

export const Duplicate_Step_1_Words = () => {
  const sidebar = tmpl(sidebarWords);
  const main = tmpl(mainEmpty);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);

  mockWords.forEach(word => {
    appendValueLineId(sidebar, "list-items", word);
  });
    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar,
        header,
        main,
        footer,
    })
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
    const el = tmpl(mainWords);

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
