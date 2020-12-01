import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockWords, mockThemes} from "./common/mock-data";
import {appendCardPairsTextText} from "./common/card-pairs";
import {appendCardPairsTextImage} from "./common/card-pairs";
import {modulePage, ModulePageKind} from "@components/module";
import {SearchWidget} from "@components/image-search";

import sidebarEmpty from "@templates/module/memory/edit/edit-new/duplicate/step1-sidebar-empty.html";
import sidebarWords from "@templates/module/memory/edit/edit-new/duplicate/step1-sidebar-words.html";
import sidebarImageWords from "@templates/module/memory/edit/edit-new/images/step1-sidebar-text.html";
import sidebarImageEmpty from "@templates/module/memory/edit/edit-new/images/step1-sidebar-empty.html";
import sidebarImageImages from "@templates/module/memory/edit/edit-new/images/step1-sidebar-images.html";
import sidebarChooseThemes from "@templates/module/memory/edit/edit-new/step2-sidebar.html";

import headerPlain from "@templates/module/memory/edit/edit-new/header-plain.html";
import headerAddpair from "@templates/module/memory/edit/edit-new/header-addpair.html";

import mainTmpl from "@templates/module/memory/edit/temp/temp-main.html";
import mainEmpty from "@templates/module/memory/edit/edit-new/main-empty.html";
import mainWords from "@templates/module/memory/edit/edit-new/duplicate/main-cards-words.html";
import mainImages from "@templates/module/memory/edit/edit-new/images/main-cards.html";

import footerPlain from "@templates/module/memory/edit/edit-new/footer-plain.html";



export default {
  title: 'Modules/Memory-Game/Edit/DuplicateNew',
}

export const Duplicate_Step_1_Empty = () => {
  const main = tmpl(mainWords);
  const header = tmpl(headerAddpair);
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

export const Images_Step_1_Words = () => {
  const sidebar = tmpl(sidebarImageWords);
  const main = tmpl(mainImages);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);

  mockWords.forEach(word => {
    appendValueLineId(sidebar, "list-items", word);
  });
    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar,
        header,
        main: makeMainImages({
            pairKind: "text-image",
            flipSecond: false,
            isEdit: true,
            themeIndex: 1
        }),
        footer,
    })
}

export const Images_Step_1_Empty = () => {
  const sidebar = tmpl(sidebarImageEmpty);
  const main = tmpl(mainEmpty);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);


    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar,
        header,
        main,
        footer,
    })
}

export const Images_Step_1_Images = () => {
  const sidebar = tmpl(sidebarImageImages);
  const main = tmpl(mainEmpty);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);
  const widget = SearchWidget({showRecent: false, showSelectedResult: false});
  appendId(sidebar, "image-search", widget);
    return modulePage({
        kind: ModulePageKind.EditPlain,
        sidebar,
        header,
        main,
        footer,
    })
}

export const Images_Step_2_Themes = () => {
  const sidebar = tmpl(sidebarChooseThemes);
  const main = tmpl(mainEmpty);
  const header = tmpl(headerPlain);
  const footer = tmpl(footerPlain);

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

function makeMainImages({pairKind, flipSecond, isEdit, themeIndex}:MainOptions):Element {
    const el = tmpl(mainWords);

    if(pairKind == "text-image") {
        appendCardPairsTextImage(el, {flipSecond, isEdit, themeIndex });
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
