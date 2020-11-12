import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import step1Page from "@templates/module/memory-game/edit/duplicate/step-1.html";
import step2Page from "@templates/module/memory-game/edit/duplicate/step-2.html";
import step4Page from "@templates/module/memory-game/edit/duplicate/step-4.html";
import step2ThemeItemSelected from "@templates/module/memory-game/edit/duplicate/step-2-theme-item-selected.html";
import step2ThemeItemDeselected from "@templates/module/memory-game/edit/duplicate/step-2-theme-item-deselected.html";
import step1Tooltip from "@templates/module/memory-game/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory-game/edit/duplicate/step-1-error.html";
import cardEditTmpl from "@templates/module/memory-game/edit/_common/memory-card.html";
import cardPlayTmpl from "@templates/module/memory-game/play/memory-card.html";

export default {
  title: 'Modules/Memory-Game/Edit/Duplicate',
}

const mockWords = ["שמש", "world", "שְׁמָע֕וּנִי", "blah blah blah"];

//Note - the `id` here must match the `memory-theme-[ID]` in memory.css
const mockThemes = [
    {
        content: "שמש",
        id: "basic",
        label: "Basic",
    },
    {
        content: "שמש",
        id: "foo",
        label: "Foo",
    },
    {
        content: "שמש",
        id: "bar",
        label: "Bar",
    },
    {
      content: 'Word',
      id: 'orange',
      label: 'Orange',
    }
];


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
export const Step2_Theme_2 = () => mockStep2(tmpl(step2Page), 2);

//export const Step4 = () => mockStep4(tmpl(step4Page), 0, false);
const makeStep4 = nCards => () =>  mockStep4(tmpl(step4Page), 0, false, nCards);
export const Step4_8Cards = makeStep4(8); 
export const Step4_10Cards = makeStep4(10); 
export const Step4_12Cards = makeStep4(12); 
export const Step4_14Cards = makeStep4(14); 
export const Step4_16Cards = makeStep4(16); 
export const Step4_18Cards = makeStep4(18); 
export const Step4_20Cards = makeStep4(20); 
export const Step4_22Cards = makeStep4(22); 
export const Step4_24Cards = makeStep4(24); 
export const Step4_26Cards = makeStep4(26); 
export const Step4_28Cards = makeStep4(28); 

export const Step4_Theme_1 = () => mockStep4(tmpl(step4Page), 1, false, 12);
export const Step4_Theme_1_Flipped = () => mockStep4(tmpl(step4Page), 1, true, 12);
//Helpers

function mockStep1(_page) {
    const page = appendMockCardsEdit(_page);
    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });

    return page;
}

function mockStep2(_page, selectedThemeIndex) {
    const page = appendMockCardsEdit(_page);

    mockThemes.forEach(({content, label, id}, idx) => {
        const item = idx === selectedThemeIndex ? tmpl(step2ThemeItemSelected) : tmpl(step2ThemeItemDeselected);
        const left = getChildId(item, "left");
        setTextId(left, "text-contents", content);
        const right = getChildId(item, "right");
        setTextId(right, "text-contents", content);

        setTextId(item, "label", label);

        toggleClasses(item, [`memory-theme-${id}`], true);

        appendId(page, "theme-items", item);
    });

    if(selectedThemeIndex) {
        const {id} = mockThemes[selectedThemeIndex];
        toggleClassesId(page, "cards", [`memory-theme-${id}`], true);
    }
    return page;
}

function mockStep4(_page, selectedThemeIndex, isFlipped, nCards) {
    const page = appendMockCardsPlay(_page, isFlipped, nCards);
    if(selectedThemeIndex) {
        const {id} = mockThemes[selectedThemeIndex];
        toggleClassesId(page, "cards", [`memory-theme-${id}`], true);
    }
    return page;
}

function appendMockCardsEdit(page) {
    mockWords.forEach(word => {
        const card = tmpl(cardEditTmpl);
        const left = getChildId(card, "left");
        setValueId(left, "text-contents", word);
        const right = getChildId(card, "right");
        setValueId(right, "text-contents", word);

        appendId(page, "cards", card);
    });

    return page;
}
function appendMockCardsPlay(page, isFlipped, nCards) {
    for(let i = 0; i < nCards; i++) {
        const word = mockWords[0];
        const card = tmpl(cardPlayTmpl);
        setTextId(card, "text-contents", word);
        appendId(page, "cards", card);

        if(isFlipped) {
            toggleClassesId(card, "flip", [`flip-card-clicked`], true);
        }
    }

    return page;
}
