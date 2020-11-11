import {renderTemplate as tmpl} from "@utils/template";
import {appendId, appendValueLineId, getChildId, setValueId, appendTextLineId, toggleClassesId, setTextId} from "@utils/dom";
import step1Page from "@templates/module/memory-game/edit/duplicate/step-1.html";
import step1Tooltip from "@templates/module/memory-game/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory-game/edit/duplicate/step-1-error.html";
import cardTmpl from "@templates/module/memory-game/edit/_common/memory-card.html";

export default {
  title: 'Modules/Memory-Game/Edit/Duplicate',
}

const mockWords = ["שמש", "world", "שְׁמָע֕וּנִי", "blah blah blah"];
export const Step1 = () => {
    const page = tmpl(step1Page, {
    });
    appendMock(page);
    return page;
}

export const Step1_Input = () => {
    const page = tmpl(step1Page, {
    });
    appendMock(page, true);
    return page;
}
export const Step1_Tooltip= () => {
    const page = tmpl(step1Page, { });

    appendId(page, "tooltip", tmpl(step1Tooltip));
    appendMock(page);
    return page;
}


export const Step1_Error = () => {
    const page = tmpl(step1Page, { });

    appendId(page, "error", tmpl(step1Error));
    appendMock(page);
    return page;
}
function appendMock(page, isInput) {

    setTextId(page, "list-items", "");

    mockWords.forEach(word => {
      //create text item
      appendValueLineId(page, "list-items", word);

      //create cards
      const card = tmpl(cardTmpl);
      const side1 = getChildId(card, "card-1");
      setValueId(side1, "label", word);
      const side2 = getChildId(card, "card-2");
      setValueId(side2, "label", word);

      appendId(page, "cards", card);
    })

}