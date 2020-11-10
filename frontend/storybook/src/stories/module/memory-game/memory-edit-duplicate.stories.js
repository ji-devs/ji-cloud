import {renderTemplate as tmpl} from "@utils/template";
import {appendId, getChildId, toggleClassesId, setTextId} from "@utils/dom";
import step1Page from "@templates/module/memory-game/edit/duplicate/step-1.html";
import step1Tooltip from "@templates/module/memory-game/edit/duplicate/step-1-tooltip.html";
import step1Error from "@templates/module/memory-game/edit/duplicate/step-1-error.html";
import listItemTmpl from "@templates/module/memory-game/edit/_common/memory-list-item.html";
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
function appendMock(page) {
    mockWords.forEach(word => {
      //create text item
      const listItem = tmpl(listItemTmpl);
      setTextId(listItem, "label", word);
      appendId(page, "list-items", listItem);

      //create cards
      const card = tmpl(cardTmpl);
      const side1 = getChildId(card, "card-1");
      setTextId(side1, "label", word);
      const side2 = getChildId(card, "card-2");
      setTextId(side2, "label", word);

      appendId(page, "cards", card);
    })

}