import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import {mockWords, mockThemes} from "./memory-common";

import step1Page from "@templates/module/memory/edit/duplicate/step-1.html";
import step2Page from "@templates/module/memory/edit/duplicate/step-2.html";
import step4Page from "@templates/module/memory/edit/duplicate/step-4.html";
import step2ThemeItemSelected from "@templates/module/memory/edit/duplicate/step-2-theme-item-selected.html";
import step2ThemeItemDeselected from "@templates/module/memory/edit/duplicate/step-2-theme-item-deselected.html";
import step1Tooltip from "@templates/module/memory/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory/edit/duplicate/step-1-error.html";
import cardEditTextTmpl from "@templates/module/memory/edit/_common/memory-card-text.html";
import cardEditPreviewTmpl from "@templates/module/memory/edit/_common/memory-card-preview.html";
import cardPlayTmpl from "@templates/module/memory/play/memory-card.html";

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
    const iframe = getChildId(page, "jig-module-iframe");
    iframe.srcdoc = "<html><body><h1>Player here!</h1></body></html>";

    return page;
}
//Helpers

function mockStep1(_page) {
    const page = appendMockCards(_page, {flipSecond: false, textInput: true});
    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });
    const {id} = mockThemes[0];
    toggleClassesId(page, "cards", [`memory-theme-${id}`], true);

    return page;
}

function mockStep2(_page, selectedThemeIndex, flipSecond) {
    const page = appendMockCards(_page, {flipSecond, textInput: false});

    mockThemes.forEach(({content, label, id}, idx) => {
        const item = idx === selectedThemeIndex ? tmpl(step2ThemeItemSelected) : tmpl(step2ThemeItemDeselected);
        const left = getChildId(item, "left");
        setTextId(left, "text-contents", content);

        setTextId(item, "label", label);

        toggleClasses(item, [`memory-theme-${id}`], true);
        
        appendId(page, "theme-items", item);
    });

    const {id} = mockThemes[selectedThemeIndex];
    toggleClassesId(page, "cards", [`memory-theme-${id}`], true);
    return page;
}

function appendMockCards(page, {flipSecond, textInput}) {
    mockWords.forEach(word => {
        const card = tmpl(textInput ? cardEditTextTmpl : cardEditPreviewTmpl);
        const left = getChildId(card, "left");
        if(textInput) {
            setValueId(left, "text-contents", word);
        } else {
            setTextId(left, "text-contents", word);
        }
        const right = getChildId(card, "right");
        if(textInput) {
            setValueId(right, "text-contents", word);
        } else {
            setTextId(right, "text-contents", word);
        }
        
        if(flipSecond) {
            toggleClasses(right, [`flip-card-clicked`], true);
        }
        appendId(page, "cards", card);
    });

    return page;
}