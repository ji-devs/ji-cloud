import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, toggleClasses, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import step1Page from "@templates/module/memory-game/edit/duplicate/step-1.html";
import step2Page from "@templates/module/memory-game/edit/duplicate/step-2.html";
import step2ThemeItemSelected from "@templates/module/memory-game/edit/duplicate/step-2-theme-item-selected.html";
import step2ThemeItemDeselected from "@templates/module/memory-game/edit/duplicate/step-2-theme-item-deselected.html";
import step1Tooltip from "@templates/module/memory-game/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory-game/edit/duplicate/step-1-error.html";
import cardTmpl from "@templates/module/memory-game/edit/_common/memory-card.html";

export default {
  title: 'Modules/Memory-Game/Edit/Duplicate',
}

const mockWords = ["שמש", "world", "שְׁמָע֕וּנִי", "blah blah blah"];
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

//Helpers

function mockStep1(_page) {
    const page = appendMockCards(_page);
    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      appendValueLineId(page, "list-items", word);
    });

    return page;
}

function mockStep2(_page, selectedThemeIndex) {
    const page = appendMockCards(_page);

    mockThemes.forEach(({content, label, id}, idx) => {
        const item = idx === selectedThemeIndex ? tmpl(step2ThemeItemSelected) : tmpl(step2ThemeItemDeselected);

        setTextId(item, "content-left", content);
        setTextId(item, "content-right", content);
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

function appendMockCards(page) {
    mockWords.forEach(word => {
      const card = tmpl(cardTmpl);
      const side1 = getChildId(card, "card-1");
      setValueId(side1, "label", word);
      const side2 = getChildId(card, "card-2");
      setValueId(side2, "label", word);

      appendId(page, "cards", card);
    });

    return page;
}