import { renderTemplate as tmpl } from "@utils/template";
import { appendId, appendValueLineId, getChildId, setValueId, appendTextLineId, setTextId } from "@utils/dom";

import {getThemeByIndex} from "../../_config/themes";
import {getInitialWords} from "../../_config/initial-words";
import { makeMainPairs } from "../_utils/main";
import { modulePage, ModulePageKind } from "@components/module";
import { SearchWidget } from "@components/image-search";

import sidebarStep1Empty from "@templates/module/memory/edit/duplicate/sidebar/step1-empty.html";
import sidebarStep1Words from "@templates/module/memory/edit/duplicate/sidebar/step1-words.html";
import headerEmpty from "@templates/module/memory/edit/_common/header/empty.html";
import headerAddPair from "@templates/module/memory/edit/_common/header/add-pair.html";
import mainEmpty from "@templates/module/memory/edit/_common/main/empty.html";
import footerDefault from "@templates/module/memory/edit/_common/footer/default.html";

export default {
  title: 'Modules/Memory/Edit/Duplicate',
}

export const Step1_Init = () => {
  const main = tmpl(mainEmpty);
  const header = tmpl(headerEmpty);
  const footer = tmpl(footerDefault);
  const sidebar = tmpl(sidebarStep1Words);

  getInitialWords().forEach(word => {
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

export const Step1_Edit = () => {
  const main = makeMainPairs({pairKind: "text-text", flipSecond: false, isEdit: true, themeIndex: 1}); 
  const header = tmpl(headerAddPair);
  const footer = tmpl(footerDefault);
  const sidebar = tmpl(sidebarStep1Empty);

  return modulePage({
    kind: ModulePageKind.EditPlain,
    sidebar,
    header,
    main,
    footer,
  })
}
/*
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
*/