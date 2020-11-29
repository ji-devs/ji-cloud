import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId, setIframeContentsId} from "@utils/dom";
import {mockWords, mockThemes} from "../common/mock-data";
import {appendCardPairsTextText} from "../common/card-pairs";

import step1Page from "@templates/module/memory/edit/duplicate/step-1.html";
import step2Page from "@templates/module/memory/edit/duplicate/step-2.html";
import step4Page from "@templates/module/memory/edit/duplicate/step-4.html";
import step1Tooltip from "@templates/module/memory/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory/edit/duplicate/step-1-error.html";
import { appendStep2Sidebar } from "../common/step-2";

export default {
  title: 'Modules/Memory-Game/Edit/Duplicate',
}


export const Step1 = () => mockStep1(tmpl(step1Page));

export const Step1_Tooltip= () => {
    const page = mockStep1(tmpl(step1Page));
    appendId(page, "tooltip", tmpl(step1Tooltip));
    return page;
}

export const Step1_Error = () => {
    const page = mockStep1(tmpl(step1Page));
    appendId(page, "error", tmpl(step1Error));
    return page;
}

export const Step2 = () => mockStep2(tmpl(step2Page), 0);

export const Step2_Theme_1 = () => mockStep2(tmpl(step2Page), 1);
export const Step2_Theme_1_FlipSecond = () => mockStep2(tmpl(step2Page), 1, true);
export const Step2_Theme_2 = () => mockStep2(tmpl(step2Page), 2);
export const Step2_Theme_2_FlipSecond = () => mockStep2(tmpl(step2Page), 2, true);

export const Step4 = () => {
    const page = tmpl(step4Page);
    setIframeContentsId(page, "module-iframe", "<html><body><h1>Player here!</h1></body></html>");

    return page;
}
//Helpers

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

function mockStep2(_page, themeIndex, flipSecond?:boolean) {
    const page = appendCardPairsTextText(_page, {
        flipSecond, 
        isEdit: false,
        themeIndex
    });

    return appendStep2Sidebar(page, {themeIndex});
}
